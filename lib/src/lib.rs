use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::Error;

pub struct Printful {
    api_key: String,
}

impl Printful {
    pub fn new(api_key: String) -> Self {
        Printful { api_key }
    }

    pub async fn get_store(self: &Self) -> Result<Store, Error> {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.printful.com/store")
            .header(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Basic {}", base64::encode(&self.api_key))).unwrap(),
            )
            .send()
            .await?;

        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());

        let printful_response = res.json::<PrintfulResponse<Store>>().await?;
        println!("Printful Response:\n{:#?}", printful_response);

        Ok(printful_response.result)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct Store {
    id: u64,
    name: String,
}

#[derive(serde::Deserialize, Debug)]
struct PrintfulResponse<R> {
    code: u16,
    result: R,
}
