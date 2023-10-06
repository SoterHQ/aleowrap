use super::{Command, CurrentNetwork};

use snarkvm::prelude::{
    query::Query,
    store::{helpers::memory::ConsensusMemory, ConsensusStore},
    Identifier, PrivateKey, ProgramID, VM, Value,
};

use anyhow::{bail, Result};
use std::str::FromStr;

pub fn execute(
    private_key: &str,
    program_id: &str,
    function: &str,
    inputs: Vec<String>,
    record: Option<&str>,
    fee: Option<u64>,
    query: Option<&str>,
) -> Result<String> {
    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)?;
    let vm = VM::from(store)?;

    let private_key = PrivateKey::from_str(private_key)?;
    let program_id = ProgramID::from_str(program_id)?;
    let function = Identifier::from_str(function)?;

    let query = match query {
        Some(query) => query,
        None => "https://vm.aleo.org/api",
    };

    // Load the program and it's imports into the process.
    Command::load_program(&query, &mut vm.process().write(), &program_id)?;

    let query = Query::from(query);

    // Prepare the fee.
    let fee_record = match record {
        Some(record_string) => Some(Command::parse_record(&private_key, &record_string)?),
        None => None,
    };

    let inputs: Vec<Value<CurrentNetwork>> = inputs.iter().map(|x| Value::from_str(x).unwrap()).collect();
    let priority_fee = fee.unwrap_or(0);
    
    // Create a new transaction.
    let transaction = vm.execute(
        &private_key,
        (program_id, function),
        inputs.iter(),
        fee_record,
        priority_fee,
        Some(query),
        rng,
    )?;

    Ok(transaction.to_string())
}
