use ggez::{event::MouseButton, GameResult};

use crate::{GameState, Position};

pub fn handle_mouse_down_event(
    game_state: &mut GameState,
    mb: MouseButton,
    x: f32,
    y: f32,
) -> GameResult {
    match mb {
        MouseButton::Left => {
            game_state.left_mouse_button_pressed = true;
            game_state.selection_rect.start = Position { x, y };
            game_state.selection_rect.end = Position { x, y };
        }
        _ => (),
    }
    Ok(())
}

pub fn handle_mouse_up_event(
    game_state: &mut GameState,
    mb: MouseButton,
    _x: f32,
    _y: f32,
) -> GameResult {
    match mb {
        MouseButton::Left => {
            game_state.left_mouse_button_pressed = false;
        }
        _ => (),
    }
    Ok(())
}
