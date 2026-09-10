mod branch;
mod cli;

use branch::GitRepository;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::Confirm;

fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args) {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), String> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner:.green} {msg}").unwrap());
    pb.set_message("Scanning branches...");

    let branches = GitRepository::list_local_branches(&args.base)?;
    pb.finish_and_clear();

    let filtered: Vec<_> = branches.into_iter().filter(|b| !b.is_current).collect();

    for branch in filtered {
        if branch.is_merged {
            println!("Found merged branch: {}", branch.name.yellow());
            if Confirm::new().with_prompt(format!("Delete {}?", branch.name)).interact().unwrap_or(false) {
                GitRepository::delete_branch(&branch.name, false)?;
            }
        }
    }
    Ok(()) 
}