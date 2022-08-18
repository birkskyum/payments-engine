use polars::prelude::DataFrame;

use crate::{
    accounts::{make_accounts, AccountList},
    util::{
        get_account, get_accounts_except_client, get_disputed, get_transaction, remove_disputed,
    },
};

use super::Transaction;

pub fn resolve(
    accounts: &DataFrame,
    disputes: &mut Vec<Transaction>,
    transaction: &Transaction,
    tx_path: &String,
) -> DataFrame {
    // check if dispute already exist
    match get_disputed(disputes, transaction) {
        // if not, return
        None => accounts.clone(),
        Some(_) => {
            // get the original transaction
            match get_transaction(transaction.tx, tx_path) {
                None => todo!(),
                Some(original_transaction) => {
                    let transaction_amount = original_transaction.get_amount();

                    // get relevant client
                    let account = get_account(accounts, transaction.client);

                    if !account.get_locked() {
                        // if it does, remove transaction from dispute
                        remove_disputed(disputes, transaction);

                        // make a new updated client, with less hold, more available
                        let new_account = make_accounts(
                            vec![transaction.client as u32],
                            vec![account.get_available() + transaction_amount],
                            vec![account.get_held() - transaction_amount],
                            vec![account.get_total()],
                            vec![account.get_locked()],
                        );

                        get_accounts_except_client(accounts, transaction).add_account(&new_account)
                    } else {
                        accounts.clone()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{accounts::load_accounts, execute_transactions::execute_transactions};

    /**
     * This test make deposits, withdrawals, disputes, and then a resolve to see if it reverses everything.
     * It also attempts to run resolve towards transactions that are not withdrawals,
     * as well as non-existing transactions and already disputed ones.
     */

    #[test]
    fn can_resolve() {
        let accounts = execute_transactions("fixtures/resolve_transactions.csv".to_string());
        let expected = load_accounts("fixtures/resolve_accounts.csv".to_string());

        assert_eq!(accounts, expected);
    }
}
