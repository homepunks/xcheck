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
    let resp: Vec<Resp> = reqwest::blocking::get(url)?.json()?;

    println!("response: {resp:#?}");

    Ok(())
}
