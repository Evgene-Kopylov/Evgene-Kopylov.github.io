use macroquad::prelude::*;

pub struct InteractableObject {
    position: Vec2,
    color: Color,
    radius: f32,
}

impl InteractableObject {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(100.0, 100.0),
            color: DARKGRAY,
            radius: 10.0,
        }
    }

    pub fn update(&mut self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            10.,
            10.,
            self.color,
        );
    }

    pub fn draw_collision(&self) {
        draw_circle_lines(
            self.position.x,
            self.position.y,
            self.radius,
            1.,
            BLUE
        )
    }
}