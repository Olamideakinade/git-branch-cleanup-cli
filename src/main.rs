mod branch;
mod cli;

use branch::GitRepository;
use clap::Parser;
use cli::Cli;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{Confirm, MultiSelect};
use regex::Regex;

fn main() {
    let args = Cli::parse();
    if let Err(e) = run(args) {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run(args: Cli) -> Result<(), String> {
    println!("\n{}", "🚀 Git Branch Cleanup CLI v1.3.0".cyan().bold());
    println!("{}", "-----------------------------------------".dimmed());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.cyan.bold} {msg}")
            .unwrap(),
    );
    pb.set_message("Scanning repository local branches...");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let branches = GitRepository::list_local_branches(&args.base)?;
    pb.finish_with_message(format!("Scanned {} local branches successfully.\n", branches.len()).green().to_string());

    if args.json {
        let json_output = serde_json::to_string_pretty(&branches)
            .map_err(|e| format!("Failed to serialize branches to JSON: {}", e))?;
        println!("{}", json_output);
        return Ok(items_processed_summary(0, 0));
    }

    let protect_regex = match &args.protect {
        Some(p) => Some(Regex::new(p).map_err(|e| format!("Invalid protect regex pattern: {}", e))?),
        None => None,
    };

    let mut candidate_branches = Vec::new();

    for branch in branches {
        if branch.is_current {
            continue;
        }

        if let Some(ref re) = protect_regex {
            if re.is_match(&branch.name) {
                continue;
            }
        }

        let mut matches_criteria = branch.is_merged;

        if let Some(days) = args.older_than {
            if let Some(date) = branch.last_commit_date {
                let age_days = (chrono::Utc::now() - date).num_days() as u64;
                if age_days >= days {
                    matches_criteria = true;
                }
            }
        }

        if matches_criteria {
            candidate_branches.push(branch);
        }
    }

    if candidate_branches.is_empty() {
        println!("{}", "✨ No stale or merged branches found matching criteria. Clean workspace!".green().bold());
        return Ok(());
    }

    println!("Found {} candidate branch(es) for cleanup:\n", candidate_branches.len().to_string().yellow().bold());

    let items: Vec<String> = candidate_branches
        .iter()
        .map(|b| {
            let status = if b.is_merged { "merged".green() } else { "stale".yellow() };
            format!("{:<30} [{}] (Author: {})", b.name.bold(), status, b.author)
        })
        .collect();

    let defaults = vec![true; candidate_branches.len()];
    let selections = MultiSelect::new()
        .with_prompt("Select branches to delete (Space to select, Enter to confirm):")
        .items(&items)
        .defaults(&defaults)
        .interact()
        .map_err(|e| format!("Interactive prompt failed: {}", e))?;

    if selections.is_empty() {
        println!("{}", "No branches selected for deletion. Exiting.".dimmed());
        return Ok(());
    }

    if args.dry_run {
        println!("\n{} Dry run mode enabled. The following branches would be deleted:", "[DRY RUN]".yellow().bold());
        for &idx in &selections {
            println!("  - {}", candidate_branches[idx].name.red());
        }
        return Ok(());
    }

    let confirm = Confirm::new()
        .with_prompt(format!("Are you sure you want to delete {} selected branch(es)?", selections.len()))
        .default(false)
        .interact()
        .map_err(|e| format!("Confirmation prompt failed: {}", e))?;

    if !confirm {
        println!("{}", "Operation cancelled by user.".red());
        return Ok(());
    }

    let pb_del = ProgressBar::new(selections.len() as u64);
    pb_del.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#=-"),
    );

    let mut deleted_count = 0;
    for &idx in &selections {
        let branch_name = &candidate_branches[idx].name;
        pb_del.set_message(format!("Deleting {}", branch_name));
        match GitRepository::delete_branch(branch_name, false) {
            Ok(_) => deleted_count += 1,
            Err(err) => eprintln!("\n{} Failed to delete {}: {}", "warn:".yellow(), branch_name, err),
        }
        pb_del.inc(1);
    }

    pb_del.finish_with_message("Cleanup completed!".green().to_string());
    println!("\n{} Successfully deleted {} branch(es).\n", "✨".green(), deleted_count.to_string().cyan().bold());

    Ok(())
}

fn items_processed_summary(_processed: usize, _deleted: usize) -> () {
    ()
}
