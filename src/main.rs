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

    let protected_patterns: Vec<String> = args.protect
        .map(|s| s.split(',').map(|p| p.trim().to_string()).collect())
        .unwrap_or_else(|| vec!["main".to_string(), "master".to_string(), "develop".to_string()]);

    let filtered: Vec<_> = branches.into_iter()
        .filter(|b| {
            if b.is_current {
                return false;
            }
            if protected_patterns.iter().any(|pat| b.name.contains(pat)) {
                return false;
            }
            if let Some(days) = args.older_than {
                if let Some(date) = b.last_commit_date {
                    let duration = chrono::Utc::now().signed_duration_since(date);
                    if duration.num_days() < days as i64 {
                        return false;
                    }
                }
            }
            true
        })
        .collect();

    if args.json {
        let json_output = serde_json::to_string_pretty(&filtered)
            .map_err(|e| format!("Failed to serialize to JSON: {}", e))?;
        println!("{}", json_output);
        return Ok(());
    }

    if filtered.is_empty() {
        println!("{}", "No branches found matching criteria.".green());
        return Ok(());
    }

    println!("{} (base: {})", "Found eligible branches:".bold(), args.base.cyan());
    for b in &filtered {
        let status = if b.is_merged { "merged".green() } else { "stale".yellow() };
        println!("  - {} [{}] (author: {})", b.name.bold(), status, b.author);
    }

    if args.dry_run {
        println!("\n{}", "[Dry-run] No branches were actually deleted.".yellow().bold());
        return Ok(());
    }

    let confirmed = if args.force {
        true
    } else {
        Confirm::new()
            .with_prompt("Do you want to delete these branches?")
            .default(false)
            .interact()
            .unwrap_or(false)
    };

    if !confirmed {
        println!("{}", "Operation cancelled.".yellow());
        return Ok(());
    }

    for b in filtered {
        print!("Deleting {}... ", b.name);
        match GitRepository::delete_branch(&b.name, args.force) {
            Ok(_) => println!("{}", "SUCCESS".green()),
            Err(e) => println!("{} ({})", "FAILED".red(), e),
        }
    }

    Ok(())
}
