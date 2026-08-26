use clap::Parser;
use gutter_detection::average;

#[derive(Parser)]
#[command(name = "cli", about = "Gutter detection CLI")]
struct Args {
    /// Input image file(s) to process
    #[arg(short = 'i', long = "input", num_args = 1..)]
    input: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for file in &args.input {
        average(file);
    }
}