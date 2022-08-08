use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::prelude::*;
use crate::rand::{
    gen_range,
    srand
};

pub struct InteractableObject {
    position: Vec2,
    color: Color,
    radius: f32,
}

fn get_random_position_on_screen() -> Vec2 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(time);
    Vec2::new(
        gen_range((0.), (screen_width())),
        gen_range((0.), screen_height())
    )
}

fn get_random_size() -> f32 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(time);
    gen_range(10., 25.)
}


impl InteractableObject {
    pub fn new() -> Self {
        Self {
            position: get_random_position_on_screen(),
            color: DARKGRAY,
            radius: get_random_size(),
        }
    }

    pub fn draw_collision(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.radius,
            self.radius,
            self.color,
        );
    }
}