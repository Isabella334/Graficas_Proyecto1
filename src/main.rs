mod framebuffer;
mod maze;
mod player;
mod caster;
mod textures;
mod sprites;
mod renderer;
mod game;
mod audio;

use game::Game;

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let block_size = 100;

    let mut game = Game::new(window_width, window_height, block_size);
    game.run();
}

