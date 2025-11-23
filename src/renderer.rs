use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;
use crate::caster::cast_ray;
use crate::sprites::Sprite;
use crate::textures::TextureManager;
use raylib::prelude::*;
use std::f32::consts::PI;

pub fn draw_sprite(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    sprite: &Sprite,
    texture_manager: &TextureManager,
) {
    let screen_width = framebuffer.width as f32;
    let screen_height = framebuffer.height as f32;

    let sprite_a = (sprite.pos.y - player.pos.y).atan2(sprite.pos.x - player.pos.x);
    let mut angle_difference = sprite_a - player.a;
    while angle_difference > PI {
        angle_difference -= 2.0 * PI
    }
    while angle_difference < -PI {
        angle_difference += 2.0 * PI
    }
    if angle_difference.abs() > player.fov / 2.9 {
        return;
    }

    let sprite_d = ((player.pos.x - sprite.pos.x).powi(2) + (player.pos.y - sprite.pos.y).powi(2)).sqrt();
    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let ray = cast_ray(framebuffer, maze, player, sprite_a, 100, false);
    let behind_wall = sprite_d >= ray.distance;
    if behind_wall {
        return;
    }

    let sprite_size = (screen_height / sprite_d) * 40.0;
    let screen_x = ((angle_difference / player.fov) + 0.5) * screen_width;

    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;

    let end_x = (start_x + sprite_size as usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size as usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            let tx = sprite.start_anim_x + ((x - start_x) as u32 * sprite.frame_width / sprite_size as u32);
            let ty = sprite.start_anim_y + ((y - start_y) as u32 * sprite.frame_height / sprite_size as u32);

            let color = texture_manager.get_pixel_color(sprite.texture_key, tx, ty);
            if color.a > 0 {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    if cell == ' ' {
        return;
    }
    framebuffer.set_current_color(Color::DARKOLIVEGREEN);

    for x in x0..x0 + block_size {
        for y in y0..y0 + block_size {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }
}

fn cell_to_texture_color(texture_cache: &TextureManager, cell: char, tx: u32, ty: u32) -> Color {
    texture_cache.get_pixel_color(cell, tx, ty)
}

pub fn render_maze(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let x0 = col_index * block_size;
            let y0 = row_index * block_size;
            draw_cell(framebuffer, x0, y0, block_size, cell);
        }
    }

    framebuffer.set_current_color(Color::RED);
    let px = player.pos.x as u32;
    let py = player.pos.y as u32;
    framebuffer.set_pixel(px, py);

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}

pub fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    texture_manager: &TextureManager,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    for y in 0..hh as usize {
        framebuffer.set_current_color(Color::GRAY);
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }

    for y in (hh as usize)..framebuffer.height as usize {
        framebuffer.set_current_color(Color::DARKRED);
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }


    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 120.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)).max(0.0) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)).min(framebuffer.height as f32) as usize;

        for y in stake_top..stake_bottom {
            let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0;

            let color = cell_to_texture_color(texture_manager, intersect.impact, intersect.tx as u32, ty as u32);
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(i, y as u32);
        }
    }
}

pub fn render_sword(framebuffer: &mut Framebuffer, texture_cache: &TextureManager) {
    let sword_width = 64;
    let sword_heigth = 64;
    let scale = 5.0;
    let display_width = (sword_width as f32 * scale) as u32;
    let display_height = (sword_heigth as f32 * scale) as u32;
    let ui_x = 10.0;
    let ui_y = (framebuffer.height - display_height) as usize;

    for ty_disp in 0..display_height {
        for tx_disp in 0..display_width {
            let tx = (tx_disp as f32 / scale) as u32;
            let ty = (ty_disp as f32 / scale) as u32;
            let color = texture_cache.get_pixel_color('s', tx, ty);
            if color.a == 0 {
                continue;
            }
            let x = ui_x as u32 + tx_disp;
            let y = ui_y as u32 + ty_disp;
            framebuffer.set_current_color(color);
            framebuffer.set_pixel(x, y);
        }
    }
}

pub fn render_minmap(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    world_block_size: usize,
    player: &Player,
    pos: Vector2
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let x0 = pos.x as usize + col_index * block_size;
            let y0 = pos.y as usize + row_index * block_size;
            draw_cell(framebuffer, x0, y0, block_size, cell);
        }
    }

    framebuffer.set_current_color(Color::BLACK);
    let scale = block_size as f32 / world_block_size as f32;

    let px = pos.x as f32 + player.pos.x * scale;
    let py = pos.y as f32 + player.pos.y * scale;

    let square_size = 5;
    for i in 1..square_size {
        for j in 1..square_size {
            framebuffer.set_pixel(px as u32 + i, py as u32 + j);
        }
    }
}

