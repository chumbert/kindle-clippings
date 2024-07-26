use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short = 't', long)]
    pub(crate) title: Option<String>,

    #[arg(short = 'a', long)]
    pub(crate) author: Option<String>,

    #[arg(last = true)]
    pub(crate) clippings_file: PathBuf,
}
