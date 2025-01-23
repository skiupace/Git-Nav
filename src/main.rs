mod ui;
mod events;
mod common;
mod folders_and_files_handler;

use ui::run;
use std::env;
use common::Result;

#[allow(unused_imports)]
use events::handle_events;

#[allow(unused_imports)]
use folders_and_files_handler::is_git_repo;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-git-repo>", args[0]);
        std::process::exit(1);
    } let repo_path = &args[1];

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, &repo_path);
    ratatui::restore();
    result
}
