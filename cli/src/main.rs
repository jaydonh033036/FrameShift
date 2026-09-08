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
    #[arg(short = 't', long = "avg-type", default_value_t = AverageType::Median, value_enum)]
    avg_type: AverageType,

    /// The width of the border to be considered
    #[arg(short = 'w', long = "border-width", default_value_t = 2)]
    border_width: u8,
}

fn main() {
    let args = Args::parse();

    for file in &args.input {
        average(file, args.avg_type, args.border_width);
    }
}