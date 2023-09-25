use super::{Command, CurrentNetwork};

use snarkvm::prelude::{
    query::Query,
    store::{helpers::memory::ConsensusMemory, ConsensusStore},
    PrivateKey, Value, VM,
};

use anyhow::{bail, Result, Context};
use std::str::FromStr;

pub fn join(
    private_key: &str,
    first_record: &str,
    second_record: &str,
    fee_record: &str,
    fee: Option<u64>,
    query: Option<&str>,
) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://vm.aleo.org/api",
    };

    // Specify the query
    let query = Query::from(query);

    // Retrieve the private key.
    let private_key = PrivateKey::from_str(private_key).context("private_key is error")?;

    println!("📦 Creating join...\n");

    // Prepare the fees.
    let fee_record = Command::parse_record(&private_key, fee_record).context("fee_record is error")?;
    let fee = match fee {
        Some(fee) => fee,
        None => 3000u64,
    };
    let fee = (fee_record, fee);

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
            Some(fee),
            Some(query),
            rng,
        )
        .context("execute error")?;

    Ok(transaction.to_string())
}