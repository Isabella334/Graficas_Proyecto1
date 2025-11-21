use caster::cast_ray;
use player::{process_events, Player};
use raylib::prelude::*;
use framebuffer::Framebuffer;
use maze::{load_maze, Maze};

mod framebuffer;
mod maze;
mod player;
mod caster;

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

fn render_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    block_size: usize,
    player: &Player,
) {
  let num_rays = framebuffer.width;

  let hh = framebuffer.height as f32 / 2.0;

  framebuffer.set_current_color(Color::WHITESMOKE);

  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32;
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    let distance_to_wall = intersect.distance;
    let distance_to_projection_plane = 70.0;
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;

    for y in stake_top..stake_bottom {
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

    let maze = load_maze("maze.txt");
    let mut player = Player {
        pos: Vector2::new(150.0, 150.0),
        a: PI / 3.0,
        fov: PI / 3.0,
    };

    while !window.window_should_close() {
    framebuffer.clear();

    process_events(&mut player, &window, &maze, block_size);

    let mut mode = "3D";

    if window.is_key_down(KeyboardKey::KEY_M) {
      mode = if mode == "2D" { "3D" } else { "2D" };
    }

    if mode == "2D" {
      render_maze(&mut framebuffer, &maze, block_size, &player);
    } else {
      render_3d(&mut framebuffer, &maze, block_size, &player);
    }

    framebuffer.swap_buffers(&mut window, &raylib_thread);

    thread::sleep(Duration::from_millis(16));
    }
}
