use polars::prelude::DataFrame;

use crate::{
    accounts::{make_accounts, AccountList},
    util::{account_exists, get_account, get_accounts_except_client},
};

use super::Transaction;

pub fn withdrawal(
    accounts: &DataFrame,
    _disputes: &[Transaction],
    transaction: &Transaction,
) -> DataFrame {
    match account_exists(accounts, transaction.client) {
        false => accounts.clone(),
        true => {
            let client = get_account(accounts, transaction.client);
            if client.get_available() >= transaction.get_amount() && !client.get_locked() {
                let new_account = make_accounts(
                    vec![transaction.client as u32],
                    vec![client.get_available() - transaction.get_amount()],
                    vec![0.0],
                    vec![client.get_total() - transaction.get_amount()],
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
     * This test covers withdrawals from existing and non-existing accounts, with amounts within and beyond whats on the available amount.
     */

    #[test]
    fn can_deposit() {
        let accounts = execute_transactions("fixtures/withdrawal_transactions.csv".to_string());
        let expected = load_accounts("fixtures/withdrawal_accounts.csv".to_string());

        assert_eq!(accounts, expected);
    }

    // fn can_withdraw() {
    //     let mut accounts = construct_accounts_dataframe();

    //     let tx1 = Transaction {
    //         _type: crate::transactions::TransactionType::Deposit,
    //         client: 5,
    //         tx: 1,
    //         amount: Some(10.1),
    //     };

    //     let tx2 = Transaction {
    //         _type: crate::transactions::TransactionType::Withdrawal,
    //         client: 1,
    //         tx: 2,
    //         amount: Some(10.3),
    //     };

    //     let tx3 = Transaction {
    //         _type: crate::transactions::TransactionType::Withdrawal,
    //         client: 5,
    //         tx: 3,
    //         amount: Some(5.2),
    //     };

    //     let tx4 = Transaction {
    //         _type: crate::transactions::TransactionType::Withdrawal,
    //         client: 5,
    //         tx: 4,
    //         amount: Some(5.1),
    //     };

    //     accounts = deposit(&accounts, &vec![], &tx1);
    //     accounts = withdrawal(&accounts, &vec![], &tx2);
    //     accounts = withdrawal(&accounts, &vec![], &tx3);
    //     accounts = withdrawal(&accounts, &vec![], &tx4);

    //     let mut accounts_after_transactions = construct_accounts_dataframe();
    //     accounts_after_transactions = accounts_after_transactions.add_account(&make_accounts(
    //         vec![5],
    //         vec![5.0],
    //         vec![0.0],
    //         vec![5.0],
    //         vec![false],
    //     ));

    //     assert_eq!(accounts, accounts_after_transactions);
    // }
}
