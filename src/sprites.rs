use raylib::prelude::*;

pub struct Sprite {
    pub pos: Vector2,
    pub texture_key: char,
    pub frame_width: u32,
    pub frame_height: u32,
    pub start_anim_x: u32,
    pub start_anim_y: u32,
}

impl Sprite {
    pub fn new(x: f32, y: f32, start_anim_x: u32, start_anim_y: u32, texture_key: char, frame_width: u32, frame_height: u32) -> Self {
        Sprite { 
            pos: Vector2::new(x, y),
            texture_key, 
            frame_width,
            frame_height,
            start_anim_x,
            start_anim_y,
        }
    }

}
