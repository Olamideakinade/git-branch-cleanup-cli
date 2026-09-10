use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "git-branch-cleanup-cli",
    author = "Open Source Developer",
    version = "0.1.0",
    about = "A fast CLI utility to clean up merged and stale git branches."
)]
pub struct Cli {
    #[arg(
        short,
        long,
        default_value = "main",
        help = "Base branch to check merge status against"
    )]
    pub base: String,

    #[arg(
        short,
        long,
        help = "Delete branches older than specified days"
    )]
    pub older_than: Option<u64>,

    #[arg(
        short,
        long,
        help = "Only show branches, do not prompt for deletion"
    )]
    pub dry_run: bool,

    #[arg(
        short,
        long,
        help = "Force deletion of unmerged branches (-D)"
    )]
    pub force: bool,

    #[arg(
        short,
        long,
        help = "Skip interactive confirmation prompts"
    )]
    pub yes: bool,

    #[arg(
        long,
        default_value = "^main$|^master$|^dev$|^staging$",
        help = "Regex pattern of branch names to protect from deletion"
    )]
    pub protect_pattern: String,
}
