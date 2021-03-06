extern crate reqwest;

use dotenv;
use printful_lib::Printful;

use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key = "PRINTFUL_API_KEY";
    let api_key = dotenv::var(key).unwrap();

    let printful = Printful::new(api_key);

    let store = printful.get_store().await?;

    println!("Store:\n{:#?}", store);

    Ok(())
}
