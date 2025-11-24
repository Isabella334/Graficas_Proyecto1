use crate::audio::Audio;
use crate::framebuffer::Framebuffer;
use crate::maze::load_maze;
use crate::player::{process_events, Player};
use crate::renderer::*;
use crate::sprites::Sprite;
use crate::textures::TextureManager;
use raylib::math::Vector2;
use raylib::prelude::*;
use std::f32::consts::PI;

pub enum GameState {
    MainMenu,
    Playing,
    Win,
    GameOver
}

pub struct Game {
    window: RaylibHandle,
    raylib_thread: RaylibThread,
    framebuffer: Framebuffer,
    player: Player,
    texture_manager: TextureManager,
    block_size: usize,
    game_state: GameState,
    minimap_pos: Vector2,
    enemies: Vec<Sprite>,
    princess: Sprite,
    audio: Audio
}

impl Game {
    pub fn new(window_width: i32, window_height: i32, block_size: usize) -> Self {
        let (mut window, raylib_thread) = raylib::init()
            .size(window_width, window_height)
            .title("Knight Maze")
            .log_level(TraceLogLevel::LOG_WARNING)
            .build();

        window.set_target_fps(60);

        let framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

        let minimap_pos = Vector2::new(1030.0, 10.0);

        let player = Player {
            pos: Vector2 {
                x: 150.0,
                y: 150.0,
            },
            a: PI / 3.0,
            fov: PI / 3.0,
            lives: 3
        };

        let texture_manager = TextureManager::new(&mut window, &raylib_thread);

        let mut enemies = Vec::new();
        enemies.push(Sprite::new(1090.0, 165.0, 0, 0, 'g', 64, 64));
        enemies.push(Sprite::new(180.0, 690.0, 0, 0, 'g', 64, 64));
        enemies.push(Sprite::new(1070.0, 590.0, 0, 0, 'g', 64, 64));
        enemies.push(Sprite::new(500.0, 420.0, 0, 0, 'g', 64, 64));

        let princess = Sprite::new(500.0, 500.0, 0, 0, 'p', 14, 35);

        Self {
            window,
            raylib_thread,
            framebuffer,
            player,
            texture_manager,
            block_size,
            game_state: GameState::MainMenu,
            minimap_pos,
            enemies,
            princess,
            audio: Audio::new()
        }
    }

    pub fn run(&mut self) {
        self.framebuffer.set_background_color(Color::BLUE);

        self.audio.play_music("assets/music.wav");
        while !self.window.window_should_close() {
            self.framebuffer.clear();

            match self.game_state {
                GameState::MainMenu => {
                    let mut d = self.window.begin_drawing(&self.raylib_thread);
                    render_start(&mut d, self.framebuffer.width, &self.texture_manager);
                    if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                        self.game_state = GameState::Playing;
                    }
                }
                GameState::Win => {
                    let mut d = self.window.begin_drawing(&self.raylib_thread);
                    render_victory(&mut d, self.framebuffer.width, &self.texture_manager);
                    if d.is_key_pressed(KeyboardKey::KEY_ENTER) {
                        self.game_state = GameState::Playing;
                    }
                }
                GameState::GameOver => {
                    let mut d = self.window.begin_drawing(&self.raylib_thread);
                    render_game_over(&mut d, self.framebuffer.width, &self.texture_manager);
                }
                GameState::Playing => {
                    let maze = load_maze("maze.txt");

                    process_events(&self.window, &mut self.player, &maze, self.block_size);

                    for enemy in &mut self.enemies {
                        let dist = self.player.pos.distance_to(enemy.pos);
                        let dx = self.player.pos.x - enemy.pos.x;
                        let dy = self.player.pos.y - enemy.pos.y;

                        if dist < 200.0 {
                            self.audio.sfx("assets/goblin.wav");
                            enemy.pos.x += dx / dist * 2.0;
                            enemy.pos.y += dy / dist * 2.0;
                        }

                        if dist < 30.0 {
                            self.player.lives -= 1;
                            self.player.pos = Vector2::new(150.0, 150.0);
                            if self.player.lives == 0 {
                                self.game_state = GameState::GameOver;
                            }
                        }
                    }

                    if self.player.pos.distance_to(self.princess.pos) < 30.0 {
                        self.game_state = GameState::Win;
                        self.audio.sfx("assets/win.wav");
                    }

                    let mut mode = "3D";
                    if self.window.is_key_down(KeyboardKey::KEY_M) {
                        mode = if mode == "2D" { "3D" } else { "2D" };
                    }

                    if mode == "2D" {
                        render_maze(&mut self.framebuffer, &maze, self.block_size, &self.player);
                    } else {
                        render_3d(&mut self.framebuffer, &maze, &self.player, self.block_size, &self.texture_manager);
                        render_minmap(&mut self.framebuffer, &maze, 20, self.block_size, &self.player, &self.princess, &mut self.enemies, self.minimap_pos);
                        render_sword(&mut self.framebuffer, &self.texture_manager);
                        render_enemies(&mut self.framebuffer, &maze, &self.player, &self.texture_manager, &mut self.enemies, &self.princess);
                        render_lives(&mut self.framebuffer, &self.texture_manager, &self.player);
                    }

                    self.framebuffer.swap_buffers(&mut self.window, &self.raylib_thread);
                }
            }
        }
    }
}

