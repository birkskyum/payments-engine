use std::io::BufWriter;

use polars::prelude::{CsvWriter, DataFrame, SerWriter};

extern crate csv;

pub fn print_accounts(accounts: &mut DataFrame) {
    let handle = std::io::stdout();
    let stream = BufWriter::new(handle);
    CsvWriter::new(stream)
        .has_header(true)
        .finish(accounts)
        .unwrap();
}
