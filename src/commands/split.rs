use super::{Command, CurrentNetwork};

use snarkvm::prelude::{
    query::Query,
    store::{helpers::memory::ConsensusMemory, ConsensusStore},
    PrivateKey, Value, VM,
};

use anyhow::{Context, Result};
use std::str::FromStr;

pub fn split(private_key: &str, record: &str, amount: u64, query: Option<&str>) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://api.explorer.aleo.org/v1",
    };

    // Specify the query
    let query = Query::from(query);

    // Retrieve the private key.
    let private_key = PrivateKey::from_str(private_key).context("private_key is error")?;

    println!("📦 Creating split...\n");

    let function = "split";

    let record = Command::parse_record(&private_key, record).context("first_record is error")?;

    let inputs = vec![
        Value::Record(record),
        Value::from_str(&format!("{}u64", amount)).context("Error amount is error")?,
    ];
    // Generate the transfer_private transaction.
    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
        .context("ConsensusStore open error")?;
    let vm = VM::from(store)?;

    // Create a new transaction.
    let transaction = vm
        .execute(
            &private_key,
            ("credits.aleo", function),
            inputs.iter(),
            None,
            0u64,
            Some(query),
            rng,
        )
        .context("execute error")?;

    Ok(transaction.to_string())
}
