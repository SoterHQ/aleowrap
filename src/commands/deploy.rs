use std::str::FromStr;

use anyhow::{anyhow, Result, Context};
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
    let private_key = PrivateKey::from_str(private_key).context("parse private_key")?;
    let program_id = ProgramID::from_str(program_id).context("parse program_id")?;

    let fee = match fee {
        Some(fee) => fee,
        None => 1000u64,
    };

    // Fetch the package from the directory.
    let package =
        Command::parse_package(program_id, Some(String::from(path))).context("Error package")?;

    println!(
        "📦 Creating deployment transaction for '{}'...\n",
        program_id.to_string()
    );

    // Generate the deployment
    let deployment = package.deploy::<CurrentAleo>(None).context("Error deploy")?;
    let deployment_id = deployment
        .to_deployment_id()
        .context("Error to_deployment_id")?;

    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
        .context("Error ConsensusStore")?;
    let vm = VM::from(store).context("Error VM")?;

    // Compute the minimum deployment cost.
    let (minimum_deployment_cost, (_, _)) =
        deployment_cost(&deployment).context("Error deployment_cost")?;
    // Determine the fee.
    let fee_in_microcredits = minimum_deployment_cost
        .checked_add(fee)
        .ok_or_else(|| anyhow!("Fee overflowed for a deployment transaction"))
        .context("Error checked_add fee")?;

    // Prepare the fees.
    let fee_record = Command::parse_record(&private_key, record).context("Error parse_record")?;
    let (_, fee) = vm
        .execute_fee_raw(
            &private_key,
            fee_record,
            fee_in_microcredits,
            deployment_id,
            Some(query),
            rng,
        )
        .context("Error execute_fee_raw")?;

    // Construct the owner.
    let owner = ProgramOwner::new(&private_key, deployment_id, rng).context("Error ProgramOwner")?;

    // Create a new transaction.
    let transaction =
        Transaction::from_deployment(owner, deployment, fee).context("Error from_deployment")?;

    Ok(transaction.to_string())
}
