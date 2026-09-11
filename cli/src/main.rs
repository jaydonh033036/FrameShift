use clap::Parser;
use gutter_detection::average;
use gutter_detection::Verbosity;

#[derive(Parser)]
#[command(name = "cli", about = "Gutter detection CLI")]
struct Args {
    /// Input image file(s) to process
    #[arg(short = 'i', long = "input", num_args = 1..)]
    input: Vec<String>,

    /// Output file(s) for the averaged-colour image. Either omit entirely (no image
    /// is saved) or provide exactly one output path per input file.
    #[arg(short = 'o', long = "output", num_args = 0..)]
    output: Vec<String>,

    /// The width of the border to be considered
    #[arg(short = 'w', long = "border-width", default_value_t = 2)]
    border_width: u8,

    /// Show a progress bar per image while scanning, then the average colour.
    #[arg(short = 'v', long = "verbose", conflicts_with = "quiet")]
    verbose: bool,

    /// Suppress all output (progress bars, pixel colours, and the average colour)
    #[arg(short = 'q', long = "quiet", conflicts_with = "verbose")]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    if !args.output.is_empty() && args.output.len() != args.input.len() {
        eprintln!(
            "Error: --output was given {} path(s) but there are {} input file(s). \
             Provide either no outputs (skip saving) or exactly one output per input.",
            args.output.len(),
            args.input.len()
        );
        std::process::exit(1);
    }

    let verbosity = if args.quiet {
        Verbosity::Quiet
    } else if args.verbose {
        Verbosity::Verbose
    } else {
        Verbosity::Normal
    };

    for (idx, file) in args.input.iter().enumerate() {
        let output_file = args.output.get(idx).map(|s| s.as_str());
        average(file, output_file, args.border_width, verbosity);
    }
}