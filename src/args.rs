use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(author, about, version, long_about = None)]
pub(crate) struct Args {
    /// path of file to open
    #[arg(short, long, default_value = None)]
    pub(crate) file_path: Option<String>,
}
