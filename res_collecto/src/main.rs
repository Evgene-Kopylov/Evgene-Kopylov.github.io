mod settings;
mod unit;
mod select_frame;

use crate::settings::*;
use crate::unit::*;
use crate::select_frame::*;

use macroquad::prelude::*;



// struct SelectorFrame {
//     point1: Vec2,
//     point2: Vec2,
//     color: Color,
// }

// impl SelectorFrame {
//     pub fn new() -> Self {
//         let mouse_position = mouse_position().into();
//         let mut color = SELECTOR_COLOR;
//         color.a = 0.14;
//         Self {
//             point1: mouse_position,
//             point2: mouse_position,
//             color,
//         }
//     }
//
//     pub fn update(&mut self, mouse_position: Vec2, unit: &mut Unit) {
//         if is_mouse_button_pressed(MouseButton::Left) {
//             self.point1 = mouse_position;
//             unit.selected = false;
//         }
//
//         if is_mouse_button_down(MouseButton::Left) {
//             self.point2 = mouse_position;
//
//             draw_rectangle(
//                 self.point1.x,
//                 self.point1.y,
//                 self.point2.x - self.point1.x,
//                 self.point2.y - self.point1.y,
//                 self.color,
//             );
//         }
//
//         // выделение области
//         if is_mouse_button_released(MouseButton::Left) {
//             if
//             unit.collision.x < self.point1.x.max(self.point2.x) &&
//                 unit.collision.x > self.point1.x.min(self.point2.x) &&
//                 unit.collision.y < self.point1.y.max(self.point2.y) &&
//                 unit.collision.y > self.point1.y.min(self.point2.y)
//             {
//                 unit.selected = true;
//             }
//         }
//
//         // одиночный клик
//         if is_mouse_button_pressed(MouseButton::Left) {
//             if
//             (mouse_position.x - unit.collision.x).powf(2f32) +
//                 (mouse_position.y - unit.collision.y).powf(2f32) < (UNIT_SIZE.1 / 2.).powf(2f32)
//             {
//                 unit.selected = true;
//             }
//         }
//
//     }
//
// }


#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("../materials/path3333.png").await.unwrap();
    let mut unit = Unit::new();
    let mut selector_frame = SelectorFrame::new();

    loop {
        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        unit.update(dt, mouse_position);
        clear_background(GROUND_COLOR);
        draw_text(
            "use LMB, RMB, Shift",
            10., 20., 30., BLACK
        );

        // отрисовка пути
        if VISUAL_DEBUG || is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
            unit.draw_path(dt)
        }
        if unit.selected {
            unit.draw_collision();
            unit.draw_path(dt)
        }
        unit.draw(texture);
        selector_frame.update(mouse_position, &mut unit);
        next_frame().await
    }
}
