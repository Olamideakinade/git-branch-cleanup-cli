use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "git-branch-cleanup-cli",
    author = "Open Source Developer",
    version = "1.3.0",
    about = "A fast CLI utility to clean up merged and stale git branches with style."
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
        default_value = "false",
        help = "Preview deletions without actually executing them"
    )]
    pub dry_run: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        help = "Export branch analysis data as JSON"
    )]
    pub json: bool,

    #[arg(
        short,
        long,
        help = "Regex pattern for branches to protect from deletion"
    )]
    pub protect: Option<String>,
}
