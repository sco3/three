#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://plus.three.ie/core/offers/top";
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    println!("{}", body);
    Ok(())
}
