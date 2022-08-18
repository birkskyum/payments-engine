use serde::Deserialize;

mod chargeback;
pub use chargeback::*;

mod deposit;
pub use deposit::*;

mod dispute;
pub use dispute::*;

mod resolve;
pub use resolve::*;

mod withdrawal;
pub use withdrawal::*;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub _type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f64>,
}

impl Transaction {
    fn get_amount(&self) -> f64 {
        self.amount.unwrap_or_else(|| {
            panic!(
                "Couldn't get transaction amount for tx: '{tx_id}'",
                tx_id = &self.tx
            )
        })
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum TransactionType {
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "chargeback")]
    Chargeback,
}
