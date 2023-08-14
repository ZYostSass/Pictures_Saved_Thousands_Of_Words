use reqwest::Client;
use serde::Deserialize;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new();

    let response = client.get("https://api.nasa.gov/planetary/apod?api_key=DEMO_KEY&start_date=2022-01-01")
        .send()
        .await?;

    let body = response.text().await?;
    let apods: Vec<APOD> = serde_json::from_str(&body).unwrap();
    println!("{:?}", apods);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct APOD {
    copyright: Option<String>,
    date: String,
    explanation: String,
    hdurl: Option<String>,
    url: String,
    media_type: String,
    service_version: String,
    title: String
}
