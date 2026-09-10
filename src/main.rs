mod branch;
mod cli;

use branch::GitRepository;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use regex::Regex;
use std::io::{self, Write};

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args) {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), String> {
    let protect_regex = Regex::new(&args.protect_pattern)
        .map_err(|e| format!("Invalid protection regex: {}", e))?;

    println!(
        "{} Inspecting local branches against base '{}'...",
        "==>".blue().bold(),
        args.base
    );

    let branches = GitRepository::list_local_branches(&args.base)?;
    let now = chrono::Utc::now();

    let mut candidates = Vec::new();

    for branch in branches {
        if branch.is_current {
            continue;
        }

        if protect_regex.is_match(&branch.name) {
            continue;
        }

        let mut reason = Vec::new();

        if branch.is_merged {
            reason.push("merged");
        }

        if let Some(days) = args.older_than {
            if let Some(date) = branch.last_commit_date {
                let age_days = (now - date).num_days();
                if age_days > days as i64 {
                    reason.push("stale");
                }
            }
        }

        if !reason.is_empty() {
            candidates.push((branch, reason));
        }
    }

    if candidates.is_empty() {
        println!("{}", "No branches found matching cleanup criteria.".green());
        return Ok(0.to_string()).map(|_| ());
    }

    println!(
        "\nFound {} candidate branch(es) for cleanup:\n",
        candidates.len().to_string().yellow().bold()
    );

    for (branch, reasons) in &candidates {
        let age_str = match branch.last_commit_date {
            Some(d) => format!("{}d ago", (now - d).num_days()),
            None => "unknown age".to_string(),
        };

        println!(
            "  {} [{}] (last commit: {}, author: {}) - reason: {}",
            branch.name.cyan().bold(),
            reasons.join(", ").yellow(),
            age_str,
            branch.author,
            reasons.join(" + ")
        );
    }

    if args.dry_run {
        println!(
            "\n{0} Dry run enabled. No branches were deleted.",
            "[INFO]".blue()
        );
        return Ok(());
    }

    let confirmed = if args.yes {
        true
    } else {
        print!(
            "\n{} Proceed with deleting these branches? [y/N]: ",
            "[CONFIRM]".yellow().bold()
        );
        io::stdout().flush().map_err(|e| e.to_string())?;

        let mut input = String::new();
        io::stdin()
            .read_line(&string_input_mut(&mut input))
            .map_err(|e| e.to_string())?;

        let trimmed = input.trim().to_lowercase();
        trimmed == "y" || trimmed == "yes"
    };

    if !confirmed {
        println!("Aborted.");
        return Ok(());
    }

    println!();
    for (branch, _) in candidates {
        print!("Deleting {}... ", branch.name);
        io::stdout().flush().map_err(|e| e.to_string())?;

        match GitRepository::delete_branch(&branch.name, args.force) {
            Ok(_) => println!("{}succeeded", "[OK]".green()),
            Err(e) => println!("{} failed ({})", "[ERR]".red(), e),
        }
    }

    println!("\n{}", "Branch cleanup completed.".green().bold());
    Ok(())
}

fn string_input_mut(s: &mut String) -> &mut String {
    s
}
