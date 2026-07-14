use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Resp {
    date: String,
    base: String,
    quote: String,
    rate: f32,
}

fn main() -> anyhow::Result<()> {
    let base_url = "https://api.frankfurter.dev/v2/rates";
    let url = format!("{}?base={}&quotes={}", base_url, "USD", "KZT");
    let mut resp: Vec<Resp> = reqwest::blocking::get(url)?.json()?;
    let resp = resp.remove(0);

    println!("[{}] {}/{} is {}", resp.date, resp.base, resp.quote, resp.rate);

    Ok(())
}
