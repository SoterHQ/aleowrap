use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use rand::{rngs::StdRng, SeedableRng};
use serde_json;

use snarkvm_circuit::Aleo;
// deployment_cost as vm_deployment_cost, execution_cost as vm_execution_cost,
use snarkvm_synthesizer::{
    process::deployment_cost as vm_deployment_cost,
    Process, Program,
};

use super::deploy::resolve_imports;

pub fn deployment_cost<A: Aleo>(
    program: &str,
    imports: Option<HashMap<String, String>>,
) -> Result<String> {
    let program = Program::from_str(program)?;
    let mut process = Process::<A::Network>::load().context("Error process load")?;
    println!("resolve_imports");
    let _ = resolve_imports(&mut process, &program, imports);
    let rng = &mut StdRng::from_entropy();

    println!("Creating deployment");
    // Generate the deployment
    let deployment = process
        .deploy::<A, _>(&program, rng)
        .context("Error process deploy")?;

    let (minimum_deployment_cost, (storage_cost, synthesis_cost, namespace_cost)) =
        vm_deployment_cost(&deployment).context("Error deployment_cost")?;

    let json_object = serde_json::json!({
        "minimum_deployment_cost":minimum_deployment_cost,
        "storage_cost":storage_cost,
        "synthesis_cost":synthesis_cost,
        "namespace_cost":namespace_cost,
    });

    Ok(json_object.to_string())
}