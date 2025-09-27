use clap::{Args, Subcommand};

#[derive(Clone, Subcommand, Debug)]
pub enum RunMode {
    Watch {
        #[command(flatten)]
        options: WatchOptions,
    },
    Normal,
}

#[derive(Clone, Debug, Args)]
pub struct WatchOptions {
    #[arg(
        short,
        long,
        default_value = "src",
        help = "Directories to watch, separated by commas"
    )]
    pub watch: String,

    #[arg(
        short,
        long,
        help = "Additional patterns to ignore, separated by commas"
    )]
    pub ignore: Option<String>,

    #[arg(short, long, help = "Clear the console on each restart")]
    pub clear: bool,

    #[arg(short, long, help = "Specify the binary to run (if not using default)")]
    pub bin: Option<String>,
}
