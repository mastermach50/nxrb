use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {

    /// Build and activate new config (default)
    #[arg(long, group = "build_mode", help_heading = "Build Mode")]
    pub switch: bool,

    /// Build and make the new config boot default
    #[arg(long, group = "build_mode", help_heading = "Build Mode")]
    pub boot: bool,

    /// Build and activate new config but don't add it to the bootloader menu
    #[arg(long, group = "build_mode", help_heading = "Build Mode")]
    pub test: bool,

    /// Use ntfy to send build status notifications
    #[arg(short, long)]
    pub notify: bool,

    /// Commit message to use
    #[arg(short, long)]
    pub message: Option<String>
}