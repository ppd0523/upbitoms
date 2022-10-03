use std::collections::HashMap;
use serde_json::Value;
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

#[derive(Debug, Serialize, Deserialize)]
struct withdraw_param {
    currency: String,
    amount: String,
    address: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct withdraw_payload {
    access_key: String,
    nonce: String,
    query_hash: String,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub currency: String,       // symbol
    pub balance: f64,         // balance
    pub locked: f64,
    pub unit_currency: String,
}

pub struct Transfer {
    types: String,
    uuid: String,
    currency: String,
    txid: String,
    state: String,
}

#[derive(Debug)]
pub struct Upbit {
    access_key: String,
    secret_key: String,
    wallet: HashMap<String, Account>,
}
impl Upbit {
    pub fn new(access_key: &str, secret_key: &str) -> Self {
        Upbit {
            wallet: HashMap::new(),
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
        }
    }

    pub async fn update_wallet(&mut self) -> bool {

        let client = reqwest::Client::new();

        let claim = json!({
            "access_key": self.access_key.to
        });

        let token = jwt::encode(
            &jwt::Header::default(),
            &Claims {
                access_key: self.access_key.to_string(),
                nonce: uuid::Uuid::new_v4().to_string(),
            },
            &jwt::EncodingKey::from_secret(self.secret_key.as_ref())
        ).unwrap();

        let res_text = client.get("https://api.upbit.com/v1/accounts")
                .header("Authorization", format!("Bearer {token}"))
                .send().await.unwrap()
                .text().await.unwrap();

        let parsed: Value = serde_json::from_str(res_text.as_str()).unwrap();
        if let Value::Array(objects) = parsed {
            for obj in objects {
                let account = Account {
                    currency: obj["currency"].to_string().trim_matches('\"').to_string(),
                    balance: obj["balance"].to_string().trim_matches('\"').parse::<f64>().unwrap(),
                    locked: obj["locked"].to_string().trim_matches('\"').parse::<f64>().unwrap(),
                    unit_currency: obj["unit_currency"].to_string().trim_matches('\"').to_string(),
                };

                self.wallet.insert(account.currency.clone(), account);
            }
        }

        true
    }

    pub fn balance(&self, symbol: &str) -> Option<&Account> {
        self.wallet.get(symbol)
    }

    pub async fn withdraw(&self, symbol: &str, amount: f64, to: &str) -> String {
        let param = withdraw_param {
            currency: symbol.to_string(),
            amount: amount.to_string(),
            address: to.to_string()
        };

        let token = jwt::encode(
            &jwt::Header::default(),
            &Claims {
                access_key: self.access_key.to_string(),
                nonce: uuid::Uuid::new_v4().to_string(),
            },
            &jwt::EncodingKey::from_secret(self.secret_key.as_ref())
        ).unwrap();

        let res_text = reqwest::Client::post("https://api.upbit.com/v1/withdraws/coin")
                .header("Authorization", format!("Bearer {token}"))

        // let res_text = client.get("https://api.upbit.com/v1/accounts")
        //         .header("Authorization", format!("Bearer {token}"))
        //         .send().await.unwrap()
        //         .text().await.unwrap();
    }
}
