use crate::accounts::{AccountList};
use polars::prelude::DataFrame;

use crate::{
    accounts::make_accounts,
    util::{get_account, get_accounts_except_client, get_disputed, get_transaction},
};

use super::Transaction;

pub fn dispute(
    accounts: &DataFrame,
    disputes: &mut Vec<Transaction>,
    transaction: &Transaction,
    tx_path: &String,
) -> DataFrame {
    // check if dispute already exist
    match get_disputed(disputes, transaction) {
        Some(_) => accounts.clone(),
        None => {
            // if not, get transaction
            match get_transaction(transaction.tx, tx_path) {
                Some(original_transaction) => {
                    let transaction_amount = original_transaction.get_amount();

                    // get relevant client
                    let account = get_account(accounts, transaction.client);

                    if !account.get_locked() {
                        // add transaction to dispute
                        disputes.push(original_transaction);

                        // make a new updated client, with less available, more hold
                        let new_account = make_accounts(
                            vec![transaction.client as u32],
                            vec![account.get_available()],
                            vec![account.get_held() + transaction_amount],
                            vec![account.get_total() + transaction_amount],
                            vec![account.get_locked()],
                        );

                        get_accounts_except_client(accounts, transaction).add_account(&new_account)
                    } else {
                        accounts.clone()
                    }
                }
                None => accounts.clone(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{accounts::load_accounts, execute_transactions::execute_transactions};

    /**
     * This test make deposits, withdrawals and then disputes to see if amounts are being put back into the "held" account.
     * Disputes are also wrongly attempted towards non-existing and non-withdrawal transactions.
     */

    #[test]
    fn can_dispute() {
        let accounts = execute_transactions("fixtures/dispute_transactions.csv".to_string());
        let expected = load_accounts("fixtures/dispute_accounts.csv".to_string());

        assert_eq!(accounts, expected);
    }
}
