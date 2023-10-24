use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use rand::{rngs::StdRng, SeedableRng};
use snarkvm::{
    prelude::{
        deployment_cost,
        query::Query,
        store::{helpers::memory::ConsensusMemory, ConsensusStore},
        transaction::Transaction,
        PrivateKey, ProgramOwner, VM,
    },
    synthesizer::{Process, Program},
};

use super::{Command, CurrentAleo, CurrentNetwork};

pub fn deploy(
    private_key: &str,
    program: &str,
    fee_record: Option<&str>,
    imports: Option<HashMap<String, String>>,
    priority_fee_in_microcredits: Option<u64>,
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

    let priority_fee_in_microcredits = priority_fee_in_microcredits.unwrap_or(0u64);

    let program = Program::from_str(program)?;

    let mut process = Process::<CurrentNetwork>::load().context("Error process load")?;
    println!("Checking program imports are valid and add them to the process");
    let _ = resolve_imports(&mut process, &program, imports);
    let rng = &mut StdRng::from_entropy();

    println!("Creating deployment");
    // Generate the deployment
    let deployment = process
        .deploy::<CurrentAleo, _>(&program, rng)
        .context("Error process deploy")?;
    let deployment_id = deployment
        .to_deployment_id()
        .context("Error to_deployment_id")?;

    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
        .context("Error ConsensusStore")?;
    let vm = VM::from(store).context("Error VM")?;

    // Compute the minimum deployment cost.
    let (base_fee_in_microcredits, (_, _)) =
        deployment_cost(&deployment).context("Error deployment_cost")?;

    // Prepare the fees.
    let fee_authorization = match fee_record {
        Some(fee_record) => {
            let fee_record = Command::parse_record(&private_key, fee_record).context("Error parse_record")?;
            vm.authorize_fee_private(
                &private_key,
                fee_record,
                base_fee_in_microcredits,
                priority_fee_in_microcredits,
                deployment_id,
                rng,
            )?
        },
        None => {
            vm.authorize_fee_public(
                &private_key,
                minimum_deployment_cost,
                priority_fee_in_microcredits,
                deployment_id,
                rng,
            )?
        },
    };
    

    let fee = vm.execute_fee_authorization(fee_authorization, Some(query), rng)?;

    // Construct the owner.
    let owner =
        ProgramOwner::new(&private_key, deployment_id, rng).context("Error ProgramOwner")?;

    // Create a new transaction.
    let transaction =
        Transaction::from_deployment(owner, deployment, fee).context("Error from_deployment")?;

    Ok(transaction.to_string())
}

pub fn resolve_imports(
    process: &mut Process<CurrentNetwork>,
    program: &Program<CurrentNetwork>,
    imports: Option<HashMap<String, String>>,
) -> Result<(), String> {
    if let Some(imports) = imports {
        program.imports().keys().try_for_each(|program_id| {
            // Get the program string
            let program_id = program_id.to_string();
            if let Some(import_string) = imports.get(&program_id) {
                if &program_id != "credits.aleo" {
                    // crate::log(&format!("Importing program: {}", program_id));
                    let import = Program::<CurrentNetwork>::from_str(&import_string)
                        .map_err(|err| err.to_string())?;
                    // If the program has imports, add them
                    resolve_imports(process, &import, Some(imports.clone()))?;
                    // If the process does not already contain the program, add it
                    if !process.contains_program(import.id()) {
                        process
                            .add_program(&import)
                            .map_err(|err| err.to_string())?;
                    }
                }
            }
            Ok::<(), String>(())
        })
    } else {
        Ok(())
    }
}
