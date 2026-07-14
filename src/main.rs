use serde::Deserialize;
use clap::Parser;

#[derive(Deserialize, Debug)]
struct Resp {
    date: String,
    base: String,
    quote: String,
    rate: f32,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "USD")]
    from: String,

    #[arg(short, long, default_value = "KZT")]
    to: String,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let base_url = "https://api.frankfurter.dev/v2/rates";
    let url = format!("{}?base={}&quotes={}", base_url, args.from, args.to);
    let mut resp: Vec<Resp> = reqwest::blocking::get(url)?.json()?;
    let resp = resp.remove(0);

    println!("[{}] {}/{} is {}", resp.date, resp.base, resp.quote, resp.rate);

    Ok(())
}
