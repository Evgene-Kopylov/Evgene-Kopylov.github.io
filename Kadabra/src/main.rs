use macroquad::prelude::*;


pub const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);

#[macroquad::main("breakout")]
async fn main() {
    loop {
        clear_background(GROUND_COLOR);
        next_frame().await
    }
}
