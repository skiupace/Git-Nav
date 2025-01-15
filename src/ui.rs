use ratatui::{
    text::{Line, Span, Text},
    style::{Color, Style},
    layout::{Constraint, Layout},
    widgets::{Block, Clear, List, ListItem, ListState, Paragraph},
    Frame,
};

use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

use std::{fs, path::Path};
use std::process::Command;
use walkdir::WalkDir;

use crate::events::{handle_events, AppState};
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

// Highlight file content using syntect
fn highlight_file_content(file_path: &str) -> Result<Text> {
    // Load syntax definitions and theme
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"]; // Choose a theme

    // Find the syntax for the file
    let syntax = ps.find_syntax_for_file(file_path)?
        .unwrap_or_else(|| ps.find_syntax_plain_text());

    // Create a highlighter
    let mut h = HighlightLines::new(syntax, theme);

    // Read the file content
    let content = fs::read_to_string(file_path)?;

    // Highlight the content
    let mut highlighted_lines = Vec::new();
    for line in LinesWithEndings::from(&content) {
        let ranges = h.highlight_line(line, &ps)?;
        let spans = ranges
            .into_iter()
            .map(|(style, text)| {
                Span::styled(
                    text.to_string(),
                    Style::default()
                        .fg(Color::Rgb(style.foreground.r, style.foreground.g, style.foreground.b))
                        .bg(Color::Reset),
                )
            })
            .collect::<Vec<_>>();
        highlighted_lines.push(Line::from(spans));
    }

    Ok(Text::from(highlighted_lines))
}

fn draw_on_clear(frame: &mut Frame, area: ratatui::layout::Rect, content: Paragraph) {
    frame.render_widget(Clear, area);
    frame.render_widget(content, area);
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

    // Render the file preview (if a file is selected)
    let preview_content = if let Some(index) = selected_index {
        if let Some(file) = files.get(index) {
            match highlight_file_content(file) {
                Ok(highlighted_text) => Paragraph::new(highlighted_text),
                Err(_) => Paragraph::new("Unable to highlight file"),
            }
        } else {
            Paragraph::new("No file selected")
        }
    } else {
        Paragraph::new("No file selected")
    }
    .block(Block::bordered().title("File Preview"));

    draw_on_clear(frame, right_area, preview_content);
}

pub fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    let files = list_files(Path::new("."));
    let mut selected_index = Some(0);

    loop {
        terminal.draw(|frame| draw(frame, &files, selected_index))?;

        match handle_events(&mut selected_index, files.len())? {
            AppState::Quit => {
                break Ok(());
            }

            AppState::OpenNvim => {
                if let Some(index) = selected_index {
                    if let Some(file) = files.get(index) {
                        // Spawn neovim as a child process
                        let mut child = Command::new("nvim")
                            .arg(file)
                            .spawn()
                            .expect("Failed to open file in neovim");

                        // Wait for the editor to close
                        child.wait().expect("Failed to wait for neovim");
                    }
                    break Ok(());
                }
            }

            AppState::KeepOpen => {}
        }
    }
}
