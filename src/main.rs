use anyhow::{anyhow, Result};
use aptos_sdk::{coin_client::CoinClient, rest_client::Client, types::account_address};
use once_cell::sync::Lazy;
use std::{error::Error, str::FromStr};
use url::Url;

// :!:>section_1c
static NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var("APTOS_NODE_URL")
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.mainnet.aptoslabs.com/v1"),
    )
    .unwrap()
});

#[tokio::main]

async fn main() {
    let rest_client: Client = Client::new(NODE_URL.clone());
    let coin_client: CoinClient = CoinClient::new(&rest_client); // <:!:section_1b

    let balance = get_native_balance(
        "0x9538c839fe490ccfaf32ad9f7491b5e84e610ff6edc110ff883f06ebde82463d",
        &coin_client,
    )
    .await;

    match balance {
        Ok(value) => println!("Balance: {}", value),
        Err(err) => println!("Error: {}", err),
    }
}

//get native coin balance

async fn get_native_balance<'a>(address: &str, client: &'a CoinClient<'a>) -> Result<u64> {
    let account = account_address::AccountAddress::from_hex_literal(address)
        .map_err(|e| anyhow!("Error in getting account: {}", e))?;
    let balance = client
        .get_account_balance(&account)
        .await
        .map_err(|e| anyhow!("Error in getting balance: {}", e))?;
    Ok(balance)
}

// //get coin balance

fn get_coin_balance(address: &str) -> Result<(), Box<dyn Error>> {
    Ok(())
}
