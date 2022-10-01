
pub struct Projectile {
    pub rotation: f32,
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub speed: f32,
}


impl Projectile {
    pub fn new(rotation: f32, position: (f32, f32), size: (f32, f32), speed: f32) -> Projectile {
        Projectile {
            rotation,
            position,
            size,
            speed,
        }
    }
}