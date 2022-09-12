use jsonwebtoken as jwt;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    access_key: String,
    nonce: String,
}

fn main() {
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

    println!("access-key: {}", access_key);
    println!("secret-key: {}", secret_key);
    println!("{}", uuid::Uuid::new_v4());

    let payload = Payload {
        access_key: access_key.to_string(),
        nonce: uuid::Uuid::new_v4().to_string(),
    };

    println!("{:?}", payload);

}
