use macroquad::prelude::*;
use settings::*;

pub struct Unit {
    pub collision: Circle,
    pub rotation: f32,
    pub order: Vec<Vec2>,
    pub selected: bool,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            collision: Circle::new(
                screen_width() * 0.5,
                screen_height() * 0.5,
                UNIT_SIZE.1 / 2.
            ),
            rotation: f32::to_radians(90.0),
            order: Vec::new(),
            selected: false,
        }
    }

    pub fn update(&mut self, dt: f32, mouse_position: Vec2) {
        // указание цели мышкой
        if self.selected && is_mouse_button_released(MouseButton::Right) {
            if is_key_down(KeyCode::LeftShift) {
                self.order.push(mouse_position);
            } else {
                self.order.clear();
                self.order.push(mouse_position);
            }
        }

        // отталкиваться от краев карты
        if self.collision.y < 1f32 {
            self.collision.y += 1f32;
        }
        if self.collision.y > screen_height() - UNIT_SIZE.1 {
            self.collision.y -= 1f32;
        }
        if self.collision.x < 1f32 {
            self.collision.x += 1f32;
        }
        if self.collision.x > screen_width() - UNIT_SIZE.1 {
            self.collision.x -= 1f32;
        }

        // поворот юнита в сторону курсора
        if self.order.len() > 0 {
            self.rotation = self.rotation % f32::to_radians(360.);
            let mut dx = self.collision.x - self.order[0].x;
            if dx == 0f32 { dx += 1f32; };

            let mut dy = self.collision.y - self.order[0].y;
            if dy == 0f32 { dy += 1f32; };

            let a;
            if dx >= 0f32 {
                a = (dy / dx).atan(); }
            else {
                a = (dy / dx).atan() - f32::to_radians(180.);
            }

            // останавливаться перед целью
            if dx.powf(2f32) + dy.powf(2f32) < (UNIT_SIZE.1 / 2.).powf(2f32) {
                self.order.remove(0);
            }
            let mut da = self.rotation - a;
            if da <= f32::to_radians(-180.) {
                da += f32::to_radians(360.)
            }
            if da > f32::to_radians(180.) {
                da -= f32::to_radians(360.)
            }
            // сохранение направления движения
            if da.abs() > f32::to_radians(9.) {
                if da > 0. {
                    self.rotation -= dt * UNIT_ROTATION_SPEED
                } else {
                    self.rotation += dt * UNIT_ROTATION_SPEED
                }
            }

            self.collision.x -= dt * UNIT_SPEED * self.rotation.cos();
            self.collision.y -= dt * UNIT_SPEED * self.rotation.sin();
        }
    }

    pub fn draw_collision(&self) {
        draw_circle_lines(
            self.collision.x,
            self.collision.y,
            self.collision.r,
            1.,
            BLUE
        )
    }

    pub fn draw_path(&self, dt: f32) {
        let mut eta: f32 = 0.0;
        for i in 0..self.order.len() {
            let x1;
            let y1;
            if i == 0 {
                x1 = self.collision.x;
                y1 = self.collision.y;
            } else {
                x1 = self.order[i-1].x;
                y1 = self.order[i-1].y;
            }
            draw_line(
                x1,
                y1,
                self.order[i].x,
                self.order[i].y,
                1f32,
                BLUE,
            );
            let dx = x1 - self.order[i].x;
            let dy = y1 - self.order[i].y;

            eta += (dx.powf(2.) + dy.powf(2.)).sqrt() * dt / UNIT_SPEED * 200.0;
            draw_text(
                format!("ETA: {:.1}", eta).as_str(),
                self.order[i].x - 12.,
                self.order[i].y - 5.,
                18.,
                BLACK
            );
        }
    }

    pub fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.collision.x - UNIT_SIZE.0 * 0.5,
            self.collision.y - UNIT_SIZE.1 * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(UNIT_SIZE.0, UNIT_SIZE.1)),
                rotation: self.rotation - f32::to_radians(90.),
                ..Default::default()
            }
        );
    }
}
