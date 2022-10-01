use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;

mod main_unit;
use main_unit::*;


#[macroquad::main("breakout")]
async fn main() {
    let main_unit_texture: Texture2D = load_texture(MAIN_UNIT_TEXTURE_PATH).await.unwrap();
    let projectile_texture = load_texture(PROJECTILE_TEXTURE_PATH).await.unwrap();
    let shoot_sound: Sound = load_sound(MAIN_UNIT_SHOOT_SOUND_ASSET).await.unwrap();

    let spawn_position = (screen_width() * 0.5, screen_height() - 130.);
    let mut main_unit = MainUnit::new(
        main_unit_texture, projectile_texture, shoot_sound, spawn_position);

    loop {
        let mouse_position: Vec2 = mouse_position().into();
        main_unit.update(get_frame_time(), mouse_position);
        clear_background(GROUND_COLOR);
        main_unit.draw();
        next_frame().await
    }
}
