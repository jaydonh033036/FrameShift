use clap::Parser;
use gutter_detection::average;
use gutter_detection::border_average::AverageType;

#[derive(Parser)]
#[command(name = "cli", about = "Gutter detection CLI")]
struct Args {
    /// Input image file(s) to process
    #[arg(short = 'i', long = "input", num_args = 1..)]
    input: Vec<String>,

    /// Averaging method to use for border colour
    #[arg(short = 't', long = "avg-type", default_value_t = AverageType::Mean, value_enum)]
    avg_type: AverageType,
}

fn main() {
    let args = Args::parse();

    for file in &args.input {
        average(file, args.avg_type);
    }
}