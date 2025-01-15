use ratatui::{
    text::Text,
    style::{Color, Style},
    layout::{Constraint, Layout},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

use std::{fs, path::Path};
use walkdir::WalkDir;

use crate::events::handle_events;
use crate::common::Result;


fn list_files(repo_path: &Path) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(repo_path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            files.push(entry.path().to_string_lossy().to_string());
        }
    }
    files
}

fn draw(frame: &mut Frame, files: &[String], selected_index: Option<usize>) {
    use Constraint::{Fill, Length, Min};

    let vertical = Layout::vertical([Length(1), Min(0)]);
    let [_title_area, main_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Length(40), Fill(1)]);
    let [left_area, right_area] = horizontal.areas(main_area);

    // Render the title
    // frame.render_widget(Block::bordered().title("Git TUI"), title_area);

    // Render the file tree
    let items: Vec<ListItem> = files.iter().map(|f| ListItem::new(f.as_str())).collect();

    let list = List::new(items)
        .block(Block::bordered().title("Files Tree"))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Blue));

    let mut list_state = ListState::default();
    list_state.select(selected_index);
    frame.render_stateful_widget(list, left_area, &mut list_state);

    // Render the file preview
    if let Some(index) = selected_index {
        if let Some(file) = files.get(index) {
            let content = fs::read_to_string(file).unwrap_or_else(|_| "Unable to read file".to_string());
            let paragraph = Paragraph::new(Text::from(content))
                .block(Block::bordered().title("File Preview"));
            frame.render_widget(paragraph, right_area);
        }
    } else {
        let paragraph = Paragraph::new("No file selected")
            .block(Block::bordered().title("File Preview"));
        frame.render_widget(paragraph, right_area);
    }
}

#[allow(dead_code)]
fn draw_on_clear() {}

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    let files = list_files(Path::new(".")); // List files in the current directory
    let mut selected_index = Some(0); // Start with the first file selected_index

    loop {
        terminal.draw(|frame| draw(frame, &files, selected_index))?;
        if handle_events(&mut selected_index, files.len())? {
            break Ok(());
        }
    }
}
