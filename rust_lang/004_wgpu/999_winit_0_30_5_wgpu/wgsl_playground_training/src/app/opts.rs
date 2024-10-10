use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Opts {
    pub wgsl_file: PathBuf,

    #[clap(short, long)]
    pub create: bool,
}
