use macroquad::prelude::*;


const GROUND_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.00);
const UNIT_COLOR: Color = WHITE;
const UNIT_SIZE: (f32, f32) = (60.0, 75.0);
const UNIT_SPEED: f32 = 200.0;
// const UNIT_ROTATION_SPEED: f32 = 4.0;
const VISUAL_DEBUG: bool = true;

struct Unit {
    collision: Circle,
    rotation: f32,
    target: Vec2,
}

impl Unit {
    pub fn new() -> Self {
        Self {
            collision: Circle::new(
                screen_width() * 0.5,
                screen_height() * 0.5,
                UNIT_SIZE.1 / 2.
            ),
            rotation: 1.57,
            target: Vec2::new(
                screen_width() * 0.5,
                screen_height() * 0.5,
            )
        }
    }

    pub fn update(&mut self, dt: f32, mouse_position: Vec2) {
        // let mut rotation = 0f32;
        // if is_key_down(KeyCode::Left) {
        //     rotation -= 1f32
        // }
        // if is_key_down(KeyCode::Right) {
        //     rotation += 1f32
        // }


        // указание цели мышкой
        if is_mouse_button_down(MouseButton::Right) || is_mouse_button_down(MouseButton::Left) {
            self.target = mouse_position;
        }


        let mut y_move = -1f32;
        if is_key_down(KeyCode::Up) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) {
            y_move += 1f32;
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
        let mut dx = self.collision.x - self.target.x;
        if dx == 0f32 { dx += 1f32; };

        let mut dy = self.collision.y - self.target.y;
        if dy == 0f32 { dy += 1f32; };

        let a;
        if dx >= 0f32 { a = (dy / dx).atan(); } else { a = (dy / dx).atan() - 3.14; }

        // останавливаться перед целью
        if dx.powf(2f32) + dy.powf(2f32) < (UNIT_SIZE.1 / 2.).powf(2f32) {
            y_move = 0f32;
        }

        self.rotation = a;
        // self.rotation += rotation * dt * UNIT_ROTATION_SPEED;
        self.collision.x += y_move * dt * UNIT_SPEED * self.rotation.cos();
        self.collision.y += y_move * dt * UNIT_SPEED * self.rotation.sin();
    }

    pub fn draw_collision(&self) {
        draw_circle_lines(
            self.collision.x,
            self.collision.y,
            self.collision.r,
            1.,
            RED
        )
    }

    pub fn draw_target_line(&self) {
        draw_line(
            self.collision.x,
            self.collision.y,
            self.target.x,
            self.target.y,
            1f32,
            RED
        )
    }

    pub fn draw(&self, texture: Texture2D) {
        draw_texture_ex(
            texture,
            self.collision.x - UNIT_SIZE.0 * 0.5,
            self.collision.y - UNIT_SIZE.1 * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(UNIT_SIZE.0, UNIT_SIZE.1)),
                rotation: self.rotation - 1.57,
                ..Default::default()
            }
        );
    }
}


#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("materials/path3332.png").await.unwrap();
    let mut unit = Unit::new();
    // println!("{}", unit.target);
    loop {
        let mouse_position: Vec2 = mouse_position().into();
        unit.update(get_frame_time(), mouse_position);
        clear_background(GROUND_COLOR);
        if VISUAL_DEBUG {
            unit.draw_collision();
            unit.draw_target_line();
        }
        unit.draw(texture);
        // draw_text(
        //     format!("X: {} Y: {}", mouse_position.x, mouse_position.y).as_str(),
        //     10., 20., 30., BLACK
        // );

        next_frame().await
    }
}
