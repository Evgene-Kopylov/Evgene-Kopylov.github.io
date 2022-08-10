use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::prelude::*;
use crate::rand::{
    gen_range,
    srand
};

pub struct InteractableObject {
    pub position: Vec2,
    pub color: Color,
    pub radius: f32,
    pub rotation: f32,
    pub sides: u8,
}

fn get_random_position_on_screen() -> Vec2 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(time);
    Vec2::new(
        gen_range(0., screen_width()),
        gen_range(0., screen_height())
    )
}

fn get_random_size() -> f32 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(time);
    let res: f32 = gen_range(10., 25.);
    res.round() as f32
}

fn get_random_in_range(low: f32, high: f32) -> f32 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    srand(time);
    let res: f32 = gen_range(low, high);
    res
}

impl InteractableObject {
    pub fn new() -> Self {
        Self {
            position: get_random_position_on_screen(),
            color: DARKGRAY,
            radius: 12.0,
            rotation: get_random_in_range(-180., 180.),
            sides: get_random_in_range(1., 6.) as u8,

            // radius: get_random_size(),
        }
    }

    pub fn draw_collision(&self) {
        draw_poly(
            self.position.x,
            self.position.y,
            self.sides,
            self.radius,
            self.rotation,
            self.color,
        )
    }
}