use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;
use projectile::*;

mod pointer;
use pointer::*;


#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("../materials/pointer/pointer_3.png").await.unwrap();
    let projectile_texture = load_texture(
        "../materials/pointer/projectile_glow_large.png").await.unwrap();
    let mut  spawn_position = (screen_width() * 0.5, screen_height() - 130.);
    let mut unit = Unit::new(texture, projectile_texture, spawn_position);

    loop {
        let mouse_position: Vec2 = mouse_position().into();
        unit.update(get_frame_time(), mouse_position);
        clear_background(GROUND_COLOR);
        unit.draw();
        next_frame().await
    }
}
