use raylib::prelude::*;
use framebuffer::Framebuffer;
use maze::Maze;

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
) {
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            let x0 = col_index * block_size;
            let y0 = row_index * block_size;
            draw_cell(framebuffer, x0, y0, block_size, cell);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
