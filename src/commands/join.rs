use super::{Command, CurrentNetwork};

use snarkvm_console::{
    account::PrivateKey,
    program::Value,
};
use snarkvm_ledger_query::Query;
use snarkvm_ledger_store::{helpers::memory::ConsensusMemory, ConsensusStore};
use snarkvm_synthesizer::VM;

use anyhow::{Context, Result};
use std::str::FromStr;

pub fn join(
    private_key: &str,
    first_record: &str,
    second_record: &str,
    fee_record: Option<&str>,
    priority_fee_in_microcredits: Option<u64>,
    query: Option<&str>,
) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://api.explorer.aleo.org/v1",
    };

    // Specify the query
    let query = Query::from(query);

    // Retrieve the private key.
    let private_key = PrivateKey::from_str(private_key).context("private_key is error")?;

    println!("📦 Creating join...\n");

    // Prepare the fees.
    let fee_record = if fee_record.is_some() {
        Some(
            Command::parse_record(&private_key, fee_record.unwrap())
                .context("fee_record is error")?,
        )
    } else {
        None
    };

    let priority_fee_in_microcredits = priority_fee_in_microcredits.unwrap_or(0);

    let function = "join";

    let first_record =
        Command::parse_record(&private_key, first_record).context("first_record is error")?;
    let second_record =
        Command::parse_record(&private_key, second_record).context("second_record is error")?;

    let inputs = vec![Value::Record(first_record), Value::Record(second_record)];

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
            fee_record,
            priority_fee_in_microcredits,
            Some(query),
            rng,
        )
        .context("execute error")?;

    Ok(transaction.to_string())
}
