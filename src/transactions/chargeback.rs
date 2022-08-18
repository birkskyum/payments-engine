use polars::prelude::DataFrame;

use crate::{
    accounts::{make_accounts, AccountList},
    util::{get_account, get_accounts_except_client, get_disputed},
};

use super::Transaction;

pub fn chargeback(
    accounts: &DataFrame,
    disputes: &[Transaction],
    transaction: &Transaction,
) -> DataFrame {
    // check if disputed
    match get_disputed(disputes, transaction) {
        None => accounts.clone(),
        Some(original_transaction) => {
            let account = get_account(accounts, original_transaction.client);
            let transaction_amount = original_transaction.get_amount();

            // check if locked
            if !&account.get_locked() {
                // make a new updated client, with less available, more hold
                let new_account = make_accounts(
                    vec![transaction.client as u32],
                    vec![account.get_available()],
                    vec![account.get_held() - transaction_amount],
                    vec![account.get_total() - transaction_amount],
                    vec![true],
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
     * This test intially make deposits, withdrawals, disputes and resolves, but then a chargeback happen,
     * and account is locked, and additional transactions are tested to not be executes.
     */

    #[test]
    fn can_chargeback() {
        let accounts = execute_transactions("fixtures/chargeback_transactions.csv".to_string());
        let expected = load_accounts("fixtures/chargeback_accounts.csv".to_string());

        assert_eq!(accounts, expected);
    }
}
