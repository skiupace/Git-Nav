use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::common::Result;


#[allow(dead_code)]
#[derive(PartialEq)]
pub enum AppState {
    KeepOpen,
    Quit
}

pub fn handle_events(selected_index: &mut Option<usize>, file_count: usize) -> Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Up => {
                if let Some(index) = selected_index {
                    if *index > 0 {
                        *selected_index = Some(*index - 1);
                    }
                }
            }
            KeyCode::Down => {
                if let Some(index) = selected_index {
                    if *index < file_count - 1 {
                        *selected_index = Some(*index + 1);
                    }
                }
            }
            KeyCode::Enter => {
                return Ok(true);
            }
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}
