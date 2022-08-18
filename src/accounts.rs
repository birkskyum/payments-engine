use std::fs::File;

use polars::prelude::CsvReader;
use polars::{
    prelude::{
        BooleanChunked, DataFrame, DataType, Field, Float64Chunked, NamedFrom, Schema, SerReader,
        TakeRandom, UInt32Chunked,
    },
    series::IntoSeries,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Account {
    client: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
}

pub trait AccountList {
    fn add_account(&self, account: &DataFrame) -> DataFrame;
    fn get_locked(&self) -> bool;
    fn get_available(&self) -> f64;
    fn get_held(&self) -> f64;
    fn get_total(&self) -> f64;
}

impl AccountList for DataFrame {
    fn add_account(&self, account: &DataFrame) -> DataFrame {
        self.vstack(account).expect("Can't modify list of accounts")
    }

    fn get_locked(&self) -> bool {
        get_bool_field(self, AccountColumn::Locked)
    }
    fn get_available(&self) -> f64 {
        get_float_field(self, AccountColumn::Available)
    }
    fn get_held(&self) -> f64 {
        get_float_field(self, AccountColumn::Held)
    }
    fn get_total(&self) -> f64 {
        get_float_field(self, AccountColumn::Total)
    }
}

pub fn construct_accounts_dataframe() -> DataFrame {
    make_accounts(vec![], vec![], vec![], vec![], vec![])
}

pub fn make_accounts(
    client: Vec<u32>,
    available: Vec<f64>,
    held: Vec<f64>,
    total: Vec<f64>,
    locked: Vec<bool>,
) -> DataFrame {
    DataFrame::new(vec![
        UInt32Chunked::new("client", client).into_series(),
        Float64Chunked::new("available", available).into_series(),
        Float64Chunked::new("held", held).into_series(),
        Float64Chunked::new("total", total).into_series(),
        BooleanChunked::new("locked", locked).into_series(),
    ])
    .expect("Cound't make empty list of accounts")
}

pub enum AccountColumn {
    Locked,
    Available,
    Held,
    Total,
    Client,
}

impl AccountColumn {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountColumn::Locked => "locked",
            AccountColumn::Available => "available",
            AccountColumn::Held => "held",
            AccountColumn::Total => "total",
            AccountColumn::Client => "client",
        }
    }
}

pub fn get_float_field(df: &DataFrame, column: AccountColumn) -> f64 {
    let column_name = column.as_str();
    let column_data = df
        .column(column_name)
        .unwrap_or_else(|_| panic!("Couldn't read '{column_name}' column."));

    let chunked_array = column_data
        .f64()
        .unwrap_or_else(|_| panic!("Couldn't unpack '{column_name}' column as float array."));
    chunked_array
        .get(0)
        .unwrap_or_else(|| panic!("Couldn't retrive value from '{column_name}' array"))
}

pub fn get_bool_field(df: &DataFrame, column: AccountColumn) -> bool {
    let column_name = column.as_str();
    let column_data = df
        .column(column_name)
        .unwrap_or_else(|_| panic!("Couldn't read '{column_name}' column."));

    let chunked_array = column_data
        .bool()
        .unwrap_or_else(|_| panic!("Couldn't unpack '{column_name}' column as boolean array."));
    chunked_array
        .get(0)
        .unwrap_or_else(|| panic!("Couldn't retrive value from '{column_name}' array"))
}

pub fn load_accounts(tx_path: String) -> DataFrame {
    let file = File::open(tx_path).expect("could not open file");

    CsvReader::new(file)
        .with_schema(&Schema::from(vec![
            Field::new("client", DataType::UInt32),
            Field::new("available", DataType::Float64),
            Field::new("held", DataType::Float64),
            Field::new("total", DataType::Float64),
            Field::new("locked", DataType::Boolean),
        ]))
        .has_header(true)
        .finish()
        .expect("Could not read csv file")
}
