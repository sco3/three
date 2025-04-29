use reqwest::Error;

async fn get_top_offers() -> Result<String, Error> {
    let url = "https://plus.three.ie/core/offers/top";

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;

    let body = response.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    match get_top_offers().await {
        Ok(body) => {
            println!("{:?}", body)
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
