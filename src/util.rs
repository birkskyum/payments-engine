use polars::{prelude::DataFrame, series::ChunkCompare};

use crate::transactions::{Transaction, TransactionType};

pub fn account_exists(accounts: &DataFrame, client: u16) -> bool {
    let client = get_account(accounts, client);

    // check if the row count is 0, to verify if the client exist
    client.shape().0 != 0
}

pub fn get_account(accounts: &DataFrame, client: u16) -> DataFrame {
    let mask = accounts
        .column("client")
        .expect("Coudn't read client column")
        .equal(client)
        .unwrap_or_else(|_| panic!("Couldn't lookup rows with client id: '{client}'"));

    // get rows that match the client id
    accounts
        .filter(&mask)
        .expect("Coudn't select client account")
}

pub fn get_accounts_except_client(accounts: &DataFrame, transaction: &Transaction) -> DataFrame {
    let mask = accounts
        .column("client")
        .expect("Coudn't read client column")
        .not_equal(transaction.client)
        .unwrap_or_else(|_| {
            panic!(
                "Couldn't lookup rows with client id: '{tx_client}'",
                tx_client = transaction.client
            )
        });

    // get rows that match the client id
    accounts
        .filter(&mask)
        .expect("Coudn't select all accounts except client")
}

pub fn get_transaction(tx: u32, tx_path: &String) -> Option<Transaction> {
    // load transaction data
    let mut reader = csv::Reader::from_path(tx_path).expect("Couldn't retrieve CSV from path");

    let mut transaction: Option<Transaction> = None;

    // go through each transaction
    for result in reader.deserialize() {
        let current_transaction: Transaction = result.expect("Cound't read transcription from csv");

        if current_transaction.tx == tx && current_transaction._type == TransactionType::Withdrawal
        {
            transaction = Some(current_transaction);
            break;
        }
    }

    transaction
}

pub fn get_disputed<'a, 'b>(
    disputes: &'a [Transaction],
    transaction: &'b Transaction,
) -> Option<&'a Transaction> {
    disputes.iter().find(|dispute| dispute.tx == transaction.tx)
}

pub fn remove_disputed(disputes: &mut Vec<Transaction>, transaction: &Transaction) {
    let index = disputes
        .iter()
        .position(|dispute| dispute.tx == transaction.tx)
        .expect("Couldn't find index for disputed transaction");
    disputes.remove(index);
}
