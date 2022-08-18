use super::Transaction;
use crate::accounts::{make_accounts};
use crate::accounts::{AccountList};
use crate::util::{account_exists, get_account, get_accounts_except_client};
use polars::prelude::DataFrame;

pub fn deposit(
    accounts: &DataFrame,
    _disputes: &[Transaction],
    transaction: &Transaction,
) -> DataFrame {
    match account_exists(accounts, transaction.client) {
        // creat an account if it doesn't exist
        false => {
            let new_client = make_accounts(
                vec![transaction.client as u32],
                vec![transaction.get_amount()],
                vec![0.0],
                vec![transaction.get_amount()],
                vec![false],
            );
            accounts.add_account(&new_client)
        }
        // deposit on an existing account
        true => {
            let client = get_account(accounts, transaction.client);

            if !client.get_locked() {
                let new_account = make_accounts(
                    vec![transaction.client as u32],
                    vec![client.get_available() + transaction.get_amount()],
                    vec![0.0],
                    vec![client.get_total() + transaction.get_amount()],
                    vec![false],
                );

                get_accounts_except_client(accounts, transaction).add_account(&new_account)
            } else {
                accounts.clone()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{accounts::load_accounts, execute_transactions::execute_transactions};

    /**
     * This test deposit to some non-existing and since existing accounts.
     */

    #[test]
    fn can_deposit() {
        let accounts = execute_transactions("fixtures/deposit_transactions.csv".to_string());
        let expected = load_accounts("fixtures/deposit_accounts.csv".to_string());

        assert_eq!(accounts, expected);
    }
}
