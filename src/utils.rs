use bevy::prelude::Vec2;
use bevy_enhanced_input::prelude::ActionState;

pub fn format_hud_text(fps: f64, movement: Vec2, jump_state: ActionState) -> String {
    format!(
        "FPS: {:>4.0}\nMove: [{:>4.2}, {:>4.2}] (WASD/Arrows/Stick)\nJump: {:?} (Space/W/Up/South)",
        fps, movement.x, movement.y, jump_state
    )
}
