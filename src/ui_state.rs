
/* 
 * Encapsulates the data and state related to drawing or rendering.
*/ 

use ratatui::{
    widgets::Paragraph, 
    buffer::Buffer, 
    layout::Rect
};

use std::path::PathBuf;
use std::time;


pub struct FrameState {
    pub buffer: Buffer,
    pub initialized: bool,
}

impl FrameState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::empty(Default::default()),
            initialized: false,
        }
    }
}

pub struct DrawableState<'a> {
    // List of files and folders
    pub items: Vec<PathBuf>,
    // Index of the selected item
    pub selected_index: Option<usize>,
    // Current path of the file
    pub current_path: PathBuf,
    // Content of the file in the terminal
    pub content: Paragraph<'a>,
    // Total area of the terminal
    pub area: Rect, 
    // Right side on the terminal
    pub right_area: Rect,
    // Left side on the terminal
    pub left_area: Rect,
    // Whether the key is held
    pub key_held: bool,
    // Threshold for the key held
    pub key_held_threshold: time::Duration,
    // Last time the key was pressed
    pub last_key_pressed: time::Instant,
    // Store the previous frame state
    pub frame_state: FrameState
}