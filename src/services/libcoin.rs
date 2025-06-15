use crate::{PANOPTICON_TOKEN, Error};
use reqwest::Client;
use once_cell::sync::Lazy;
use serde::Serialize;

const API_KEY_HEADER: &str = "ApiKey";
static HTTP_CLIENT: Lazy<Client> = Lazy::new(Client::new);

#[derive(Serialize)]
struct LibcoinTransactionPayload {
    #[serde(rename = "UserId")]
    user_id: String,
    #[serde(rename = "Amount")]
    amount: f64,
    #[serde(rename = "Message")]
    message: String,
}

pub async fn get_libcoin_balance(user_id: u64) -> Result<f64, Error> {
    let url = format!("https://panopticon.cacheblasters.com/libcoin/{}", user_id);
    let response_text = HTTP_CLIENT
        .get(&url)
        .header(API_KEY_HEADER, PANOPTICON_TOKEN.as_str())
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let balance: f64 = response_text.trim().parse()?;

    Ok(balance)
}

pub async fn deduct_libcoin(user_id: u64, amount: f64, message: &str) -> Result<(), Error> {
    let url = "https://panopticon.cacheblasters.com/libcoin/deduct".to_string();

    libcoin_transaction(user_id, amount, message, &url).await
    .map_err(|e| Error::from(format!("Failed to deduct libcoin: {}", e)))
}

pub async fn grant_libcoin(user_id: u64, amount: f64, message: &str) -> Result<(), Error> {
    let url = "https://panopticon.cacheblasters.com/libcoin/grant".to_string();

    libcoin_transaction(user_id, amount, message, &url).await
    .map_err(|e| Error::from(format!("Failed to grant libcoin: {}", e)))
}

async fn libcoin_transaction(user_id: u64, amount: f64, message: &str, url: &str) -> Result<(), Error> {
    let payload = LibcoinTransactionPayload {
        user_id: user_id.to_string(),
        amount,
        message: message.to_string(),
    };

    let response = HTTP_CLIENT
        .post(url)
        .header(API_KEY_HEADER, PANOPTICON_TOKEN.as_str())
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        Ok(())
    } else {
        let error_body = response.text().await.unwrap_or_else(|e| format!("Failed to read error body: {}", e));
        Err(Error::from(format!("Libcoin transaction failed: {}", error_body)))
    }
}