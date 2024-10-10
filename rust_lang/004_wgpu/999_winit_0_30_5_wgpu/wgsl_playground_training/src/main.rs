use std::{fs::OpenOptions, io::Write, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
struct Opts {
    wgsl_file: PathBuf,

    #[clap(short, long)]
    create: bool,
}

fn main() {
    wgpu_subscriber::initialize_default_subscriber(None);
    let opts = Opts::parse();

    if opts.create {
        let mut file = if let Ok(file) = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(opts.wgsl_file.clone())
        {
            file
        } else {
            println!(
                "Could't create file {:?}, make sure it doesn't already exist.",
                &opts.wgsl_file
            );
            return;
        };
        file.write_all(include_bytes!("frag.default.wgsl")).unwrap();
    }
}
