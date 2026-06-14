
use clap::Parser;

fn main() {
    let args = Args::parse();
    
    println!("{args:?}");
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    currency: String,
    #[arg(short, long)]
    base: String
}

#[derive(Debug)]
enum Currency {
    EUR,
    GBP,
    USD,
}
