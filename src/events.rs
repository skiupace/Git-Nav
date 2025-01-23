use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::common::Result;


#[derive(PartialEq)]
pub enum AppState {
    KeepOpen,
    OpenNvim,
    GoBack,
    Quit
}

pub fn handle_events(selected_index: &mut Option<usize>, file_count: usize, key_held: &mut bool) -> Result<AppState> {
    if let Event::Key(key) = event::read()? {
        match key.kind {
            KeyEventKind::Press => {
                *key_held = true;

                match key.code {
                    KeyCode::Char('q') => {
                        return Ok(AppState::Quit);
                    }

                    KeyCode::Up => {
                        if let Some(index) = selected_index {
                            if *index > 0 {
                                *selected_index = Some(*index - 1);
                            }
                        } return Ok(AppState::KeepOpen);
                    }

                    KeyCode::Down => {
                        if let Some(index) = selected_index {
                            if *index < file_count - 1 {
                                *selected_index = Some(*index + 1);
                            }
                        } return Ok(AppState::KeepOpen);
                    }

                    KeyCode::Enter => {
                        return Ok(AppState::OpenNvim);
                    }

                    KeyCode::Tab => {
                        return Ok(AppState::GoBack);
                    }

                    _ => {}
                }
            }

            KeyEventKind::Release => {
                *key_held = false;
            }
            _ => {}
        }
    }

    Ok(AppState::KeepOpen)
}
