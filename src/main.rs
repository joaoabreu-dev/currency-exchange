
use clap::Parser;
use tokio;

use currency_exchange::{ fetch_currency_data, fetch_data };

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    for arg in args.currencies.iter() {
        println!("Currency: {}", arg);
    }
    
    let url = String::from("https://api.frankfurter.dev/v1/latest?from=EUR&to=USD,GBP");

    let _ = fetch_data(url).await;

}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    currencies: Vec<String>,
    #[arg(short, long, default_value_t = String::from("USD"))]
    base: String
}

#[derive(Debug)]
enum Currency {
    EUR,
    GBP,
    USD,
}

