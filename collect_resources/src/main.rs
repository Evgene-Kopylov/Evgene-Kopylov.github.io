use macroquad::prelude::*;
mod settings;
use crate::settings::*;

mod unit;
use unit::Unit;

mod select_frame;
use select_frame::SelectorFrame;

mod interactables;

use crate::interactables::InteractableObject;



#[macroquad::main("breakout")]
async fn main() {
    let texture: Texture2D = load_texture("../assets/path3333.png").await.unwrap();
    let mut unit = Unit::new();
    let mut selector_frame = SelectorFrame::new();
    let mut reclaimables: Vec<InteractableObject> = Vec::new();
    for i in 0..30 {
        let rec = InteractableObject::new(i as u64);
        reclaimables.push(rec);
    }

    loop {
        clear_background(GROUND_COLOR);
        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        unit.update(dt, mouse_position, &mut reclaimables);
        for rec in &reclaimables {
            rec.draw_collision();
        }

        if unit.selected || is_key_down(KeyCode::LeftShift) {
            unit.draw_collision();
            unit.draw_path(dt)
        }
        unit.draw(texture);
        selector_frame.update(mouse_position, &mut unit);
        next_frame().await
    }
}
