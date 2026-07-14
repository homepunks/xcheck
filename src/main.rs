use serde::Deserialize;
use clap::Parser;

#[derive(Deserialize, Debug)]
struct Resp {
    date: String,
    base: String,
    quote: String,
    rate: f64,
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
    let from = args.from.to_uppercase();
    let to = args.to.to_uppercase();

    let base_url = "https://api.frankfurter.dev/v2/rates";
    let url = format!("{}?base={}&quotes={}", base_url, from, to);
    let [resp]: [Resp; 1] = reqwest::blocking::get(url)?
        .error_for_status()?
        .json::<Vec<Resp>>()?
        .try_into()
        .map_err(|_| anyhow::anyhow!("expected exactly one rate"))?;

    println!("[{}] {}/{} is {}", resp.date, resp.base, resp.quote, resp.rate);

    Ok(())
}
