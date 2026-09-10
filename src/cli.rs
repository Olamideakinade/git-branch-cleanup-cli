use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "git-branch-cleanup-cli",
    author = "Open Source Developer",
    version = "1.2.0",
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
        help = "Force deletion without confirmation"
    )]
    pub force: bool,

    #[arg(
        long,
        help = "Preview deletions without executing them"
    )]
    pub dry_run: bool,

    #[arg(
        long,
        help = "Output branch inspection data as JSON"
    )]
    pub json: bool,

    #[arg(
        short,
        long,
        help = "Protected branch patterns (comma separated)"
    )]
    pub protect: Option<String>,
}
