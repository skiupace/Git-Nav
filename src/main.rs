use std::env;
use std::path::PathBuf;

use git_nav::ui::run;
use git_nav::common::Result;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let repo_path = if args.len() < 2 {
        // Get current directory if no path provided
        env::current_dir()?
    } else {
        PathBuf::from(&args[1])
    };

    let mut terminal = ratatui::init();
    let result = run(&mut terminal, repo_path.to_str().unwrap_or("."));
    ratatui::restore();
    result
}
