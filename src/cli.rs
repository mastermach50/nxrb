use clap::{Parser};

#[derive(Parser, Debug, Clone)]
#[command(version, author, about, long_about = None)]
pub struct Args {

    /// Build and activate new config (default)
    // #[arg(long, group = "build_mode", help_heading = "Build Mode (defaults to switch)")]
    // pub switch: bool,

    /// Build and make the new config boot default
    #[arg(long, group = "build_mode", help_heading = "Build Mode (defaults to switch)")]
    pub boot: bool,

    /// Build and activate new config but don't add it to the bootloader menu
    #[arg(long, group = "build_mode", help_heading = "Build Mode (defaults to switch)")]
    pub test: bool,

    /// Use ntfy to send build status notifications
    #[arg(short, long)]
    pub notify: bool,

    /// Update the flake before running
    #[arg(short, long)]
    pub update: bool,

    /// Commit message to use
    #[arg(short, long)]
    pub message: Option<String>
}

impl Args {
    pub fn get_build_mode<'a>(self) -> &'a str  {
        if self.boot {
            return "boot";
        } else if self.test {
            return "test";
        } else {
            return "switch";
        }
    }
}