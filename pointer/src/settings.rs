use macroquad::prelude::Color;


pub const GROUND_COLOR: Color = Color::new(0.77, 0.8, 0.8, 1.00);
pub const UNIT_COLOR: Color = Color::new(0.94, 0.94, 0.94, 1.);
pub const PROJECTILE_TEXTURE_PATH: &str = "../materials/pointer/projectile_glow_large.png";
pub const PROJECTILE_COLOR: Color = Color::new(1.00, 0.96, 0.84, 1.00);
pub const MAIN_UNIT_SHOOT_DELAY: f32 = 0.1;
pub const MAIN_UNIT_SHOOT_SOUND_VOLUME: f32 = 0.16;
pub const MAIN_UNIT_SHOOT_SOUND_ASSET: &str = "../materials/sound/XAL_Weapon.wav";
// pub const MAIN_UNIT_SHOOT_SOUND_ASSET: &str = "../materials/sound/XAL_Weapon.wav" // Хорошо
// pub const MAIN_UNIT_SHOOT_SOUND_ASSET: &str = "../materials/sound/URLWeapon.wav" // Хорошо
// pub const MAIN_UNIT_SHOOT_SOUND_ASSET: &str ="../materials/sound/URSWeapon.wav" // Очень хорошо
pub const MAIN_UNIT_TEXTURE_PATH: &str = "../materials/pointer/pointer_3.png";


