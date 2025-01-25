
/* 
 * Encapsulates the data and state related to drawing or rendering.
*/ 

use std::path::PathBuf;
use crate::debug::Debugger;
use ratatui::widgets::Paragraph;
use ratatui::layout::Rect;

pub struct DrawableState<'a> {
    pub items: Vec<PathBuf>,
    pub selected_index: Option<usize>,
    pub current_path: PathBuf,
    pub debug: Debugger,
    pub content: Paragraph<'a>,
    // Total area of the terminal
    pub area: Rect, 
    // Right side on the terminal
    pub right_area: Rect,
    // Left side on the terminal
    pub left_area: Rect
}