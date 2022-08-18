use polars::prelude::DataFrame;

use crate::{accounts::construct_accounts_dataframe, transactions::{Transaction, TransactionType, deposit, chargeback, dispute, withdrawal, resolve}};


pub fn execute_transactions(tx_path: String) -> DataFrame {
    
  let mut accounts = construct_accounts_dataframe();
  let mut disputes: Vec<Transaction> = vec![];

  let mut reader = csv::Reader::from_path(&tx_path).expect("Can't open csv at path");

  for result in reader.deserialize() {
      let transaction: Transaction = result.expect("Couldn't read transaction from csv");

      accounts = match transaction._type {
          TransactionType::Chargeback => chargeback(&accounts, &disputes, &transaction),
          TransactionType::Deposit => deposit(&accounts, &disputes, &transaction),
          TransactionType::Dispute => dispute(&accounts, &mut disputes, &transaction, &tx_path),
          TransactionType::Resolve => resolve(&accounts, &mut disputes, &transaction, &tx_path),
          TransactionType::Withdrawal => withdrawal(&accounts, &disputes, &transaction),
      };
  }

  accounts
}

