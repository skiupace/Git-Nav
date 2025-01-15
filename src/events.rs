use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::common::Result;


#[allow(dead_code)]
#[derive(PartialEq)]
pub enum AppState {
    KeepOpen,
    OpenNvim,
    Quit
}

pub fn handle_events(selected_index: &mut Option<usize>, file_count: usize) -> Result<AppState> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(AppState::Quit),
            KeyCode::Up => {
                if let Some(index) = selected_index {
                    if *index > 0 {
                        *selected_index = Some(*index - 1);
                    }
                }
                return Ok(AppState::KeepOpen);
            }

            KeyCode::Down => {
                if let Some(index) = selected_index {
                    if *index < file_count - 1 {
                        *selected_index = Some(*index + 1);
                    }
                }
                return Ok(AppState::KeepOpen);
            }

            KeyCode::Enter => {
                return Ok(AppState::OpenNvim);
            }
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(AppState::Quit)
}
