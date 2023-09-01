use clap::Parser;
use propagate::dataset::Dataset;

#[derive(Debug, Parser)]
#[command(
    author = "tobias@tobias.dev",
    version,
    about = "
    =====================================================
    PPPP   RRRR    OOO   PPPP   A   GGGG   A   TTTTT EEEE
    P   P  R   R  O   O  P   P A A  G     A A    T   E
    PPPP   RRRR   O   O  PPPP  AAAA G  GG AAAA   T   EEEE
    P      R  R   O   O  P    A   A G   G A   A  T   E
    P      R   R   OOO   P    A   A  GGG  A   A  T   EEEE
    =====================================================

    Propagate: a simple synthetic data generator."
)]
struct Args {
    #[arg(short, long, help = "Column names to generate")]
    columns: String,

    #[arg(
        short,
        long,
        help = "Ranges for each column separated by comma (e.g. 0:100,5:25))"
    )]
    ranges: String,

    #[arg(short, long, default_value_t = 50, help = "Number of rows to generate")]
    num_rows: i32,

    #[arg(short, long, default_value = "data.csv", help = "Output file name")]
    output: String,

    #[arg(short, long, default_value = ", ", help = "Seperator for output file")]
    sep: String,
}

fn main() {
    let args: Args = Args::parse();

    let colnames: Vec<String> = args
        .columns
        .split(',')
        .map(|s: &str| s.to_string())
        .collect();
    let ranges: Vec<String> = args
        .ranges
        .split(',')
        .map(|s: &str| s.to_string())
        .collect();

    let dataset: Dataset = Dataset::build(colnames, ranges, args.num_rows, args.sep, args.output);
    println!("{:?}", dataset);
    dataset.generate_dataset();
}
