use ratatui::{
    text::Text,
    style::{Color, Style},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

use std::path::{Path, PathBuf};
use std::process::Command;
use crossterm::terminal;

use crate::files::{
    is_git_repo, 
    get_name, 
    get_icon, 
    list_files_and_folders, 
    highlight_file_content
};

use crate::events::{handle_events, AppState};
use crate::common::Result;


fn draw_on_clear(frame: &mut Frame, area: Rect, content: Paragraph) {
    frame.render_widget(Clear, area);
    frame.render_widget(content, area);
}

fn draw(frame: &mut Frame, items: &[PathBuf], selected_index: Option<usize>, current_path: &Path) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [_title_area, main_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Length(40), Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    // Render the file tree with names and icons
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|path| {
            let name = get_name(path);
            let icon = get_icon(path);
            ListItem::new(format!("{} {}", icon, name)) // Combine icon and name
        })
        .collect();

    let list = List::new(list_items)
        .block(
            Block::bordered().title(
                format!("Files Tree: {}", 
                    current_path.file_name()
                        .unwrap_or_default() // Handle cases where there's no file name
                        .to_string_lossy()
                )
            )
        )
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Blue));

    let mut list_state = ListState::default();
    list_state.select(selected_index);
    frame.render_stateful_widget(list, left_area, &mut list_state);

    // Render the file preview (if a file is selected)
    let preview_content = match selected_index.and_then(|i| items.get(i)) {
        Some(file) if file.is_file() => {
            let file_name = get_name(file);
            let content = file.to_str()
                .and_then(|path| highlight_file_content(path).ok())
                .unwrap_or_else(|| Text::from("Unable to read file"));
        
            Paragraph::new(content)
                .block(Block::bordered().title(format!("File Preview: {file_name}")))
        }

        Some(_) => {  // Directory selected
            Paragraph::new("Select a file to preview")
                .block(Block::bordered().title("File Preview"))
        }

        None => {  // No valid selection
            Paragraph::new(selected_index
                .map(|_| "No file selected")
                .unwrap_or("Select a file to view it")
            )
            .block(Block::bordered().title("File Preview"))
        }
    };
    
    draw_on_clear(frame, right_area, preview_content);
}

pub fn run(terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>, repo_path: &str) -> Result<()> {
    // Validate the Git repository path
    let path = Path::new(repo_path);

    if !is_git_repo(path) {
        return Err(format!("Not a git repository: {}", repo_path).into());
    }

    let mut current_path = path.to_path_buf();
    let mut history: Vec<PathBuf> = Vec::new(); // Track directory history
    let mut items = list_files_and_folders(&current_path);
    let mut selected_index = Some(0);
    let mut key_held = false;

    loop {
        terminal.draw(|frame| draw(frame, &items, selected_index, &current_path))?;

        match handle_events(&mut selected_index, items.len(), &mut key_held)? {
            AppState::Quit => {
                break Ok(());
            }

            AppState::OpenNvim => {
                if let Some(index) = selected_index {
                    if let Some(file) = items.get(index) {
                        if file.is_file() {
                            // Save terminal state
                            terminal.clear()?;
                            terminal::disable_raw_mode()?;
                            terminal.show_cursor()?;

                            // Spawn nvim and wait for it to complete
                            Command::new("nvim")
                                .arg(file.to_str().unwrap_or(""))
                                .status()
                                .expect("Failed to open file in neovim");

                            // Restore terminal state
                            terminal::enable_raw_mode()?;
                            terminal.hide_cursor()?;
                            terminal.clear()?;
                        } else if file.is_dir() {
                            // Navigate into the folder
                            history.push(current_path.clone());
                            current_path = file.clone();
                            items = list_files_and_folders(&current_path);
                            selected_index = Some(0);
                        }
                    }
                }
            }

            AppState::GoBack => {
                // Navigate back to the previous folder
                if let Some(prev_path) = history.pop() {
                    current_path = prev_path;
                    items = list_files_and_folders(&current_path);
                    selected_index = Some(0);
                }
            }

            AppState::KeepOpen => {}
        }
    }
}
