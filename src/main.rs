use std::env;
use git_tui::ui::run;
use git_tui::common::Result;


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
