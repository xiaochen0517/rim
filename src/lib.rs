use clap::Parser;

mod args;

pub fn run() {
    let args = args::Args::parse();
    let name = args.file_path.unwrap_or_else(|| "world".to_string());
    println!("file path : {}!", name);
}