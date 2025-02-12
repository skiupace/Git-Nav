use ratatui::{
    Frame,
    text::Text,
    style::{Color, Style},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, List, ListItem, ListState, Paragraph},
};

use crossterm::{
    cursor,
    terminal,
    QueueableCommand,
};

use std::process::Command;
use std::io::{self, Write};
use std::{path::{Path, PathBuf}, thread, time};

use crate::files::{
    get_icon, 
    get_name, 
    is_git_repo, 
    highlight_file_content, 
    list_files_and_folders
};

use crate::common::Result;
use crate::events::{handle_events, AppState};
use crate::ui_state::{DrawableState, FrameState};


fn render_frame_diff(state: &mut DrawableState, frame: &mut Frame, stdout: &mut impl Write) -> Result<()> {
    let current_buffer = frame.buffer_mut();
    
    if !state.frame_state.initialized {
        // First frame - draw everything
        stdout.queue(cursor::MoveTo(0, 0))?;
        terminal::enable_raw_mode()?;
        // Just store the buffer for future diffs
        state.frame_state.buffer = current_buffer.clone();
        state.frame_state.initialized = true;
    } else {
        // Calculate and draw only the differences
        let previous_buffer = &state.frame_state.buffer;
        
        for y in 0..current_buffer.area().height {
            for x in 0..current_buffer.area().width {
                let current_cell = &current_buffer[(x, y)];
                let previous_cell = &previous_buffer[(x, y)];
                
                if current_cell != previous_cell {
                    stdout.queue(cursor::MoveTo(x, y))?;
                    write!(stdout, "{}", current_cell.symbol())?;
                }
            }
        }
    }
    
    stdout.flush()?;
    state.frame_state.buffer = current_buffer.clone();
    Ok(())
}

fn draw_right_side(state: &mut DrawableState, frame: &mut Frame) {
    let preview_content = match state.selected_index.and_then(|i| state.items.get(i)) {
        Some(file) if file.is_file() => {
            let file_name = get_name(file);
            let content = file
                .to_str()
                .and_then(|path| highlight_file_content(path).ok())
                .unwrap_or_else(|| Text::from("Unable to read file"));

            Paragraph::new(content)
                .block(Block::bordered().title(format!("File Preview: {file_name}")))
        }

        Some(_) => {
            // Directory selected
            Paragraph::new("Select a file to preview")
                .block(Block::bordered().title("File Preview"))
        }

        None => {
            // No valid selection
            Paragraph::new(
                state
                    .selected_index
                    .map(|_| "No file selected")
                    .unwrap_or("Select a file to view it"),
            )
            .block(Block::bordered().title("File Preview"))
        }
    };

    frame.render_widget(preview_content, state.right_area);
}

fn draw_left_side(state: &mut DrawableState, frame: &mut Frame) {
    let list_items: Vec<ListItem> = state
        .items
        .iter()
        .map(|path| {
            let name = get_name(path);
            let icon = get_icon(path);
            ListItem::new(format!("{} {}", icon, name)) // Combine icon and name
        })
        .collect();

    // List component
    let list = List::new(list_items)
        .block(Block::bordered()
        .title(format!("Files Tree: {}", 
                state.current_path.file_name()
                    .unwrap_or_default() // Handle cases where there's no file name
                    .to_string_lossy(),
            )))
        // Highlight style
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Blue));

    let mut list_state = ListState::default();
    list_state.select(state.selected_index);

    frame.render_stateful_widget(list, state.left_area, &mut list_state);
}

fn draw(state: &mut DrawableState, frame: &mut Frame) {
    use Constraint::{Percentage, Min};

    // Calculate the areas of the terminal
    let vertical: Layout = Layout::vertical([Min(0)]);
    let [main_area] = vertical.areas(frame.area());
    
    // Use percentage-based horizontal split for more flexibility
    let horizontal = Layout::horizontal([
        Percentage(30),  // Left panel takes 30% of the width
        Percentage(70),  // Right panel takes 70% of the width
    ]);
    let [left_area, right_area] = horizontal.areas(main_area);

    // Assign the areas to the state
    state.area = main_area;
    state.left_area = left_area;
    state.right_area = right_area;

    // Left side | Render the file tree
    draw_left_side(state, frame);

    // Right side | Render the file preview (if a file is selected)
    draw_right_side(state, frame);
}

pub fn run(terminal: &mut ratatui::Terminal<impl ratatui::backend::Backend>, repo_path: &str) -> Result<()> {
    // Validate the Git repository path
    let path = Path::new(repo_path);

    if !is_git_repo(path) {
        return Err(format!("Not a git repository: {}", repo_path).into());
    }

    let mut history: Vec<PathBuf> = Vec::new(); // Track directory history

    // Initialize drawable state
    let mut state = DrawableState {
        items: list_files_and_folders(&path.to_path_buf()),
        selected_index: Some(0),
        current_path: path.to_path_buf(),
        area: Rect::new(0, 0, 0, 0),
        right_area: Rect::new(0, 0, 0, 0),
        left_area: Rect::new(0, 0, 0, 0),
        content: Paragraph::new(""),
        key_held: false,
        key_held_threshold: time::Duration::from_millis(100),
        last_key_pressed: time::Instant::now(),
        frame_state: FrameState::new()
    };

    // Event loop
    let mut stdout = io::stdout();
    loop {
        // Draw the current state
        terminal.draw(|frame| {
            draw(&mut state, frame);
            render_frame_diff(&mut state, frame, &mut stdout).unwrap();
        })?;

        // Handle user input and update state
        match handle_events(&mut state)? {
            AppState::Quit => {
                break Ok(());
            }

            AppState::OpenNvim => {
                if let Some(index) = state.selected_index {
                    if let Some(file) = state.items.get(index) {
                        if file.is_file() {
                            // Save terminal state
                            if cfg!(target_os = "windows") {
                                terminal.clear()?;
                            } else {
                                stdout.queue(cursor::MoveTo(0, 0))?;
                            }
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
                        }
                    }
                }
            }

            AppState::GoBack => {
                if cfg!(target_os = "windows") {
                    terminal.clear()?;
                } else {
                    stdout.queue(cursor::MoveTo(0, 0))?;
                }
                if let Some(prev_path) = history.pop() {
                    state.current_path = prev_path;
                    state.items = list_files_and_folders(&state.current_path);
                    state.selected_index = Some(0);
                }
            }

            AppState::GoForward => {
                if cfg!(target_os = "windows") {
                    terminal.clear()?;
                } else {
                    stdout.queue(cursor::MoveTo(0, 0))?;
                }
                if let Some(index) = state.selected_index {
                    if let Some(path) = state.items.get(index) {
                        if path.is_dir() {
                            history.push(state.current_path.clone());
                            state.current_path = path.clone();
                            state.items = list_files_and_folders(&state.current_path);
                            state.selected_index = Some(0);
                        }
                    }
                }
            }

            AppState::KeepOpen => {
                if state.selected_index.is_some() {
                    if cfg!(target_os = "windows") {
                        terminal.clear()?;
                    } else {
                        stdout.queue(cursor::MoveTo(0, 0))?;
                    }
                }
            }

            AppState::Relax => {
                let elapsed = state.last_key_pressed.elapsed();
                let time_left = state
                    .key_held_threshold
                    .checked_sub(elapsed)
                    .unwrap_or_else(|| time::Duration::from_millis(0));
                thread::sleep(time_left);
            }
        }
    }
}
