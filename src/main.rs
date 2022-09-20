mod args;
mod Upbit;

use jsonwebtoken as jwt;
use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    access_key: String,
    nonce: String,
}

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

    let claims = Claims {
        access_key: access_key.to_string(),
        nonce: uuid::Uuid::new_v4().to_string(),
    };

    let token = jwt::encode(
        &jwt::Header::default(),
        &claims,
        &jwt::EncodingKey::from_secret(secret_key.as_ref())
    )?;

    let client = reqwest::Client::new();
    let res_text = client.get("https://api.upbit.com/v1/accounts")
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    let mut upbit_wallet: HashMap<String, Upbit::Account> = HashMap::new();

    let parsed: Value = serde_json::from_str(res_text.as_str()).unwrap();
    if let Value::Array(objects) = parsed {
        for obj in objects {
            let account = Upbit::Account {
                currency: obj["currency"].to_string().trim_matches('\"').to_string(),
                balance: obj["balance"].to_string().trim_matches('\"').parse::<f64>().unwrap(),
                locked: obj["locked"].to_string().trim_matches('\"').parse::<f64>().unwrap(),
                unit_currency: obj["unit_currency"].to_string().trim_matches('\"').to_string(),
            };

            upbit_wallet.insert(account.currency.clone(), account);
        }
    }

    println!("{:?}", upbit_wallet);
    println!("{:?}", upbit_wallet.get("SOL").unwrap().balance);


    // println!("{}", parsed[0]);
    // println!("{}", parsed[1]);
    // println!("{}", parsed[2]);


    Ok(())
}
