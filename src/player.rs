use raylib::prelude::*;
use std::f32::consts::PI;

use crate::maze::{get_cell, Maze};

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
    pub lives: u32
}

pub fn process_events(rl: &RaylibHandle, player: &mut Player, maze: &Maze, block_size: usize) {
    const MOVE_SPEED: f32 = 5.0;
    const ROTATION_SPEED: f32 = PI / 25.0;

    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.a += ROTATION_SPEED;
    }
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        player.a -= ROTATION_SPEED;
    }

    if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        let new_x = player.pos.x - MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y - MOVE_SPEED * player.a.sin();

        let cell = get_cell(&maze, new_x, new_y, block_size);
        if let Some(c) = cell {
            if c == ' ' {
                player.pos.x -= MOVE_SPEED * player.a.cos();
                player.pos.y -= MOVE_SPEED * player.a.sin();
            } 
        }
    }
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        let new_x = player.pos.x + MOVE_SPEED * player.a.cos();
        let new_y = player.pos.y + MOVE_SPEED * player.a.sin();

        let cell = get_cell(&maze, new_x, new_y, block_size);
        if let Some(c) = cell {
            if c == ' ' {
                player.pos.x += MOVE_SPEED * player.a.cos();
                player.pos.y += MOVE_SPEED * player.a.sin();
            } 
        }
    }
}

