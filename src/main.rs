mod args;
mod Upbit;

use jsonwebtoken as jwt;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;
use serde_json::Value;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let conf = ini::ini!("./key.ini");
    let upbit = conf.get("upbit").expect("Not in upbit section");

    let access_key = upbit
            .get("access-key")
            .expect("No 'access-key'")
            .as_ref()
            .unwrap()
            .as_str();
    let secret_key = upbit
            .get("secret-key")
            .expect("No 'secret-key'")
            .as_ref()
            .unwrap()
            .as_str();

    let mut upbit = Upbit::Upbit::new(access_key, secret_key);

    upbit.update_wallet().await;

    println!("{}", upbit.balance("SOL").unwrap().balance);

    Ok(())
}
