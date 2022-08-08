use macroquad::prelude::*;
use settings::*;

mod unit;
use unit::Unit;

mod select_frame;
use select_frame::SelectorFrame;


#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("../materials/path3338.png").await.unwrap();
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
