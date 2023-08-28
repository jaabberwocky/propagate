use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    author = "tobias@tobias.dev",
    version,
    about = "Propagate: a simple synthetic data generator."
)]
struct Args {
    #[arg(short, long, help = "Column names to generate")]
    columns: String,

    #[arg(short, long, default_value_t = 50)]
    rows: u8,

    #[arg(short, long, default_value = "data.csv")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let colnames: Vec<&str> = args.columns.split(',').collect();
    for col in colnames {
        println!("{}", col);
    }
}
