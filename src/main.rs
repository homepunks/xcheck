use clap::Parser;
use serde::Deserialize;

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

mod ansi {
    pub const RESET: &str = "\x1b[0m";

    pub const GRAY: &str = "\x1b[38;2;168;153;132m";
    pub const GREEN: &str = "\x1b[38;2;184;187;38m";
    pub const YELLOW: &str = "\x1b[38;2;250;189;47m";
    pub const AQUA: &str = "\x1b[38;2;142;192;124m";
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
        println!("{aqua}1 {quote} is precisely {rate:.2} {base}!{reset}",
            aqua = ansi::AQUA,
            quote = resp.quote,
            rate = 1.0 / resp.rate,
            base = resp.base,
            reset = ansi::RESET,
        );
    }

    Ok(())
}
