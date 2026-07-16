use clap::{Args, Parser, Subcommand};
use serde::Deserialize;
use chrono::{Local, Duration};
use textplots::{Chart, Shape, Plot};

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
    #[command(subcommand)]
    command: Option<Command>,

    #[command(flatten)]
    currencies: Currencies,
}

#[derive(Args)]
struct Currencies {
    #[arg(short, long, default_value = "USD")]
    from: String,

    #[arg(short, long, default_value = "KZT")]
    to: String,
}

#[derive(Subcommand)]
enum Command {
    Stat { days: u32 },
}

mod ansi {
    pub const RESET: &str = "\x1b[0m";

    pub const GRAY: &str = "\x1b[38;2;168;153;132m";
    pub const GREEN: &str = "\x1b[38;2;184;187;38m";
    pub const YELLOW: &str = "\x1b[38;2;250;189;47m";
    pub const AQUA: &str = "\x1b[38;2;142;192;124m";
}

fn main() -> anyhow::Result<()> {
    let base_url = "https://api.frankfurter.dev/v2/rates";
    let args = Cli::parse();
    let from = args.currencies.from.to_uppercase();
    let to = args.currencies.to.to_uppercase();

    match args.command {
        Some(Command::Stat { days }) => {
            let now    = Local::now().date_naive();
            let since  = now - Duration::days(days as i64);
            let url = format!("{}?from={}&base={}&quotes={}", base_url, since.format("%Y-%m-%d"), from, to);
            let resp = reqwest::blocking::get(url)?
                .error_for_status()?
                .json::<Vec<Resp>>()?;
  
            let rates: Vec<_> = resp.iter().map(|r| r.rate).collect();
            let points: Vec<_> = rates.into_iter().enumerate().map(|(i, rate)| (i as f32, rate as f32)).collect();

            let xmax = points.len().saturating_sub(1) as f32;
            Chart::new(180, 60, 0.0, xmax).lineplot(&Shape::Lines(&points)).nice();
        }
        None => {
            let url = format!("{}?base={}&quotes={}", base_url, from, to);
            let [resp]: [Resp; 1] = reqwest::blocking::get(url)?
                .error_for_status()?
                .json::<Vec<Resp>>()?
                .try_into()
                .map_err(|_| anyhow::anyhow!("expected exactly one rate"))?;

            println!(
                "{gray}[{date}]{reset} {yellow}{base}/{quote}{reset} {gray}->{reset} {green}{rate:.2}{reset}",
                gray = ansi::GRAY,
                yellow = ansi::YELLOW,
                green = ansi::GREEN,
                reset = ansi::RESET,
                date = resp.date,
                base = resp.base,
                quote = resp.quote,
                rate = resp.rate,
            );

            if resp.rate < 1.0 && &to == "KZT" {
                println!(
                    "{aqua}1 {quote} is precisely {rate:.2} {base}!{reset}",
                    aqua = ansi::AQUA,
                    quote = resp.quote,
                    rate = 1.0 / resp.rate,
                    base = resp.base,
                    reset = ansi::RESET,
                );
            }
        }
    }

    Ok(())
}
