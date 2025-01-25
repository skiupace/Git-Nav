use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::common::Result;
use crate::ui_state::DrawableState;
use std::time;
#[derive(PartialEq)]
pub enum AppState {
    KeepOpen,
    OpenNvim,
    GoForward,
    GoBack,
    Relax,
    Quit
}

pub fn handle_events(state: &mut DrawableState) -> Result<AppState> {
    if let Event::Key(key) = event::read()? {
        match key.kind {
            KeyEventKind::Press => {
                // Only handle key presses after the threshold which is 100ms
                if state.last_key_pressed.elapsed() <= state.key_held_threshold {
                    return Ok(AppState::Relax);
                }

                // Reset the last key pressed time
                state.last_key_pressed = time::Instant::now(); 

                // Set the key held to true
                state.key_held = true;


                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(AppState::Quit);
                    }

                    KeyCode::Up => {
                        if state.selected_index.is_some() && state.selected_index.unwrap() > 0 {
                            state.selected_index = Some(state.selected_index.unwrap() - 1);
                        }
                        return Ok(AppState::KeepOpen);
                    }

                    KeyCode::Down => {
                        if state.selected_index.is_some() && state.selected_index.unwrap() < state.items.len() - 1 {
                            state.selected_index = Some(state.selected_index.unwrap() + 1);
                        }
                        return Ok(AppState::KeepOpen);
                    }

                    KeyCode::Enter => {
                        return Ok(AppState::OpenNvim);
                    }

                    KeyCode::Left => {
                        return Ok(AppState::GoBack);
                    }

                    KeyCode::Right => {
                        return Ok(AppState::GoForward);
                    }

                    _ => {}
                }
            }

            KeyEventKind::Release => {
                state.key_held = false;
            }
            _ => {}
        }
    }

    Ok(AppState::KeepOpen)
}
