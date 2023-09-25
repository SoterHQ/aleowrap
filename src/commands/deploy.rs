use std::str::FromStr;

use anyhow::{anyhow, Result};
use snarkvm::prelude::{
    deployment_cost,
    query::Query,
    store::{helpers::memory::ConsensusMemory, ConsensusStore},
    transaction::Transaction,
    PrivateKey, ProgramID, ProgramOwner, VM,
};

use super::{Command, CurrentAleo, CurrentNetwork};

pub fn deploy(
    private_key: &str,
    program_id: &str,
    path: &str,
    record: &str,
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
    let private_key = PrivateKey::from_str(private_key).expect("parse private_key");
    let program_id = ProgramID::from_str(program_id).expect("parse program_id");

    let fee = match fee {
        Some(fee) => fee,
        None => 1000u64,
    };

    // Fetch the package from the directory.
    let package =
        Command::parse_package(program_id, Some(String::from(path))).expect("Error package");

    println!(
        "📦 Creating deployment transaction for '{}'...\n",
        program_id.to_string()
    );

    // Generate the deployment
    let deployment = package.deploy::<CurrentAleo>(None).expect("Error deploy");
    let deployment_id = deployment
        .to_deployment_id()
        .expect("Error to_deployment_id");

    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
        .expect("Error ConsensusStore");
    let vm = VM::from(store).expect("Error VM");

    // Compute the minimum deployment cost.
    let (minimum_deployment_cost, (_, _)) =
        deployment_cost(&deployment).expect("Error deployment_cost");
    // Determine the fee.
    let fee_in_microcredits = minimum_deployment_cost
        .checked_add(fee)
        .ok_or_else(|| anyhow!("Fee overflowed for a deployment transaction"))
        .expect("Error checked_add fee");

    // Prepare the fees.
    let fee_record = Command::parse_record(&private_key, record).expect("Error parse_record");
    let (_, fee) = vm
        .execute_fee_raw(
            &private_key,
            fee_record,
            fee_in_microcredits,
            deployment_id,
            Some(query),
            rng,
        )
        .expect("Error execute_fee_raw");

    // Construct the owner.
    let owner = ProgramOwner::new(&private_key, deployment_id, rng).expect("Error ProgramOwner");

    // Create a new transaction.
    let transaction =
        Transaction::from_deployment(owner, deployment, fee).expect("Error from_deployment");

    Ok(transaction.to_string())
}
