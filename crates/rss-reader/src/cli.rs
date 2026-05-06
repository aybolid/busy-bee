use std::path::PathBuf;

#[derive(Debug, Clone, clap::Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
    #[arg(long, default_value = PathBuf::from("./config.toml").into_os_string())]
    pub config: PathBuf,
}

#[derive(Debug, clap::Subcommand, Default, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    #[default]
    Run,
    Init,
}
