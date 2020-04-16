extern crate base64;
extern crate reqwest;

use dotenv;

use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::Error;

#[derive(serde::Deserialize, Debug)]
struct PrintfulResponse<R> {
    code: u16,
    result: R,
}

#[derive(serde::Deserialize, Debug)]
struct Store {
    id: u64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = "PRINTFUL_API_KEY";
    let api_key = dotenv::var(key).unwrap();
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.printful.com/store")
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", base64::encode(api_key))).unwrap(),
        )
        .send()
        .await?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let printful_response = res.json::<PrintfulResponse<Store>>().await?;
    println!("Printful Response:\n{:#?}", printful_response);

    Ok(())
}
