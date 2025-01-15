mod ui;
mod events;
mod common;

use ui::run;
use common::Result;
#[allow(unused_imports)]
use events::handle_events;


fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}
