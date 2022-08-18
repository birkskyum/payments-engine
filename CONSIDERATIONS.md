# Considerations

### Crates

##### [clap](https://crates.io/crates/clap) - Command Line Argument Parser

To make a user friendly CLI, even in simple situations, the popular library _clap_ is the best tool for the job. I use it to generate the `--help` section to the cli, get the provided path to the transaction file, and let a user know if it's missing.

##### [polars](https://crates.io/crates/polars) - DataFrame Library based on Apache Arrow

Having a good dataframe makes all the difference for data driven applications. Polars is to a large extend immutable, so evey change to the dataframe happens by generating a new one and releasing the old - this will in many cases allow for parallelization. It's also columnar because it's build on Apache Arror, and that allows for great performance - dispite demanding an entire different mindset when using it instead of row-based solutions. The list of accounts in this library is a dataframe, where the transactions are not sice they are streamed from the csv to avoid keeping them all in memory at a time.

##### [serde](https://crates.io/crates/serde) - serialization/deserialization

The best libarry out there for serialization, deserialization. I use it to read the transacrions, and through Polars, I use it to output the accounts dataframe as csv formatted data to the stdout bufwriter as well.

##### [csv](https://crates.io/crates/csv) - read/write csv files

It used to accounts data in csv-format in output[]().rs and to read accounts data in csv-format for testing each type of transacrion in _load_accounts_ in the _accounts[]().rs_ file.

### Overall structure

Each type of transactions have it's own function, that will take a reference to the existing list of accounts and disputes, and a execute the transaction. A new list of accounts is being returned. I chose this structure because it's easy to test each type of transaction, and because it avoids having too much responsibility in a single file.

### Testing

Testing is done by executing a set of transactions, and compaing the account output to the expected output.
For this a set of fixtures are use, which can be found in the ./fixtures directory.