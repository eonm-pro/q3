use clap::Parser;

/// A fictional versioning CLI
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "q³")]
#[command(about = "lorem", long_about = Some("q³ helps you to build higher dimension queries"))]
pub struct Cli {
    #[arg(help = "Path to the q3 file", name = "query.q3")]
    pub path: std::path::PathBuf,
    #[arg(long, short, help = "get a request by it's id", name = "ID")]
    pub get: Option<String>,
}
