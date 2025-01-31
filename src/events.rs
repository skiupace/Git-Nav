use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::ui_state::DrawableState;
use crate::common::Result;
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


                // match state.input_mode 
                match key.code {
                    KeyCode::Char('q') => Ok(AppState::Quit),
                    KeyCode::Up => handle_up(state),
                    KeyCode::Down => handle_down(state),
                    KeyCode::Enter => Ok(AppState::OpenNvim),
                    KeyCode::Left => Ok(AppState::GoBack),
                    KeyCode::Right => Ok(AppState::GoForward),
                    KeyCode::Char('k') => handle_up(state),
                    KeyCode::Char('j') => handle_down(state),
                    KeyCode::Char('h') => Ok(AppState::GoBack),
                    KeyCode::Char('l') => Ok(AppState::GoForward),
                    KeyCode::Char('o') => Ok(AppState::OpenNvim),
                    _ => Ok(AppState::KeepOpen),
                }
            }

            KeyEventKind::Release => {
                state.key_held = false;
                Ok(AppState::KeepOpen)
            }
            _ => Ok(AppState::KeepOpen),
        }
    } else {
        Ok(AppState::KeepOpen)
    }
}

fn handle_up(state: &mut DrawableState) -> Result<AppState> {
    if state.selected_index.is_some() && state.selected_index.unwrap() > 0 {
        state.selected_index = Some(state.selected_index.unwrap() - 1);
    } Ok(AppState::KeepOpen)
}

fn handle_down(state: &mut DrawableState) -> Result<AppState> {
    // !This condition causes panic when a directory is empty.
    if state.selected_index.is_some() && state.selected_index.unwrap() < state.items.len() - 1 {
        state.selected_index = Some(state.selected_index.unwrap() + 1);
    } Ok(AppState::KeepOpen)
}
