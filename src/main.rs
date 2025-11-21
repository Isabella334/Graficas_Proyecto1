use caster::cast_ray;
use player::{process_events, Player};
use raylib::prelude::*;
use framebuffer::Framebuffer;
use maze::{load_maze, Maze};
use textures::TextureManager;

mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;

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

fn cell_to_texture_color(texture_cache: &TextureManager, cell: char, tx: u32, ty: u32) -> Color {
    texture_cache.get_pixel_color(cell, tx, ty)
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
        framebuffer.set_current_color(Color::SKYBLUE);
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }

    for y in (hh as usize)..framebuffer.height as usize {
        framebuffer.set_current_color(Color::DARKGREEN);
        for x in 0..framebuffer.width {
            framebuffer.set_pixel(x as u32, y as u32);
        }
    }


    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

        let distance_to_wall = intersect.distance;
        let distance_to_projection_plane = 70.0;
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

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    let texture_manager = TextureManager::new(&mut window, &raylib_thread);

    let maze = load_maze("maze.txt");
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    while !window.window_should_close() {
        framebuffer.clear();

        let mut mode = "3D";

        if window.is_key_down(KeyboardKey::KEY_M) {
          mode = if mode == "2D" { "3D" } else { "2D" };
        }

        if mode == "2D" {
          render_maze(&mut framebuffer, &maze, block_size, &player);
        } else {
          render_3d(&mut framebuffer, &maze, block_size, &player, &texture_manager);
        }

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
