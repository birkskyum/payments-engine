use clap::Parser;

#[derive(Parser, Default, Debug)]
pub struct Arguments {
    pub transactions_file_path: String,
}
