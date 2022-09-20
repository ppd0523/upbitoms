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