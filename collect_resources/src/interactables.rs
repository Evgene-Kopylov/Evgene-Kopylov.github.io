use macroquad::prelude::*;
use macroquad::time::get_time;
use crate::rand::{
    gen_range,
    srand
};

pub struct InteractableObject {
    pub uid: String,
    // pub state: InteractableObjectState,
    pub position: Vec2,
    pub color: Color,
    pub radius: f32,
    pub rotation: f32,
    pub sides: u8,
}


impl InteractableObject {
    pub fn new(random_seed: u64) -> Self {
        srand(random_seed);
        println!("seed{}@{}", random_seed, get_time());
        Self {
            uid: format!("seed{}@{}", random_seed, get_time()),
            // state: InteractableObjectState::Raw,
            position: Vec2::new(
                gen_range(0., screen_width()),
                gen_range(0., screen_height())
            ),
            color: DARKGRAY,
            radius: 12.0,
            rotation: gen_range(-180., 180.),
            sides: gen_range(1., 6.) as u8,
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