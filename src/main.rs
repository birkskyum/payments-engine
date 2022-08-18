use clap::Parser;
use payments_engine::clap::Arguments;
use payments_engine::execute_transactions::execute_transactions;
use payments_engine::output::{print_accounts};

fn main() {
    let cli_args = Arguments::parse();
    let tx_path = cli_args.transactions_file_path;

    let mut accounts = execute_transactions(tx_path);

    print_accounts(&mut accounts);
}
