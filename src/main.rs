use jsonwebtoken as jwt;
use serde::{
    Deserialize,
    Serialize,
};

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
    ).unwrap();

    let client = reqwest::Client::new();
    let res = client.get("https://api.upbit.com/v1/accounts")
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await?
            .text()
            .await;

    println!("{}", res.unwrap());

    Ok(())
}
