// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use rand::rngs::StdRng;
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snarkvm_circuit::Aleo;
use snarkvm_console::program::Network;
use snarkvm_console::program::ProgramID;
use snarkvm_console::program::ProgramOwner;
use snarkvm_ledger_block::Transaction;
use snarkvm_synthesizer::Process;
use snarkvm_synthesizer::Program;
use std::collections::HashMap;
use std::str::FromStr;

use crate::resolve_imports;
use crate::Command;

use anyhow::{Context, Result};
use snarkvm_ledger_query::Query;
use snarkvm_ledger_store::{helpers::memory::ConsensusMemory, ConsensusStore};
use snarkvm_synthesizer::Authorization as Authorization_VM;
use snarkvm_synthesizer::VM;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Authorization<N: Network>(Authorization_VM<N>);

impl<N: Network> Authorization<N> {
    /// Create a authorization from a string
    ///
    /// @param {string} authorization String representation of a authorization
    /// @returns {authorization | Error}
    pub fn from_string(authorization: &str) -> Result<Authorization<N>, String> {
        Authorization::from_str(authorization)
    }

    /// Get the authorization as a string. If you want to submit this authorization to the Aleo Network
    /// this function will create the string that should be submitted in the `POST` data.
    ///
    /// @returns {string} String representation of the authorization
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn from_json(json: &str) -> Result<Authorization<N>> {
        let authorization: Authorization_VM<N> = serde_json::from_str(json)?;
        Ok(Authorization(authorization))
    }
}

impl<N: Network> From<Authorization<N>> for Authorization_VM<N> {
    fn from(authorization: Authorization<N>) -> Self {
        authorization.0
    }
}

impl<N: Network> From<Authorization_VM<N>> for Authorization<N> {
    fn from(authorization: Authorization_VM<N>) -> Self {
        Self(authorization)
    }
}

impl<N: Network> FromStr for Authorization<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            Authorization_VM::<N>::from_str(s).map_err(|e| e.to_string())?,
        ))
    }
}

pub fn transaction_for_authorize<A: Aleo>(
    program_id: &str,
    execute_authorization_str: &str,
    fee_authorization_str: &str,
    query: Option<&str>,
) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://mainnet.sotertech.io",
    };

    // Initialize the VM.
    let store = ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(None)?;
    let vm = VM::from(store)?;

    let program_id = ProgramID::from_str(program_id)?;
    // Load the program and it's imports into the process.
    Command::load_program(&query, &mut vm.process().write(), &program_id)
        .context("load program error")?;

    // Specify the query
    let query = Query::from(query);

    // Generate the transfer_private transaction.
    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    let execute_authorization =
        Authorization::<A::Network>::from_json(&reorder(execute_authorization_str))
            .context("execute authorization error")?;
    let fee_authorization: Option<Authorization_VM<A::Network>> =
        if fee_authorization_str.is_empty() {
            None
        } else {
            let fee_authorization = Authorization::<A::Network>::from_json(fee_authorization_str)
                .context("fee authorization error")?;
            Some(Authorization_VM::from(fee_authorization))
        };

    let transaction = vm
        .execute_authorization(
            Authorization_VM::<A::Network>::from(execute_authorization),
            fee_authorization,
            Some(query),
            rng,
        )
        .context("execute error")?;

    Ok(transaction.to_string())
}

pub fn deploy_for_authorize<A: Aleo>(
    program: &str,
    imports: Option<HashMap<String, String>>,
    owner_str: &str,
    fee_authorization_str: &str,
    query: Option<&str>,
) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://mainnet.sotertech.io",
    };

    // Specify the query
    let query = Query::from(query);

    let program = Program::from_str(program)?;

    let mut process = Process::<A::Network>::load().context("Error process load")?;
    println!("Checking program imports are valid and add them to the process");
    let _ = resolve_imports(&mut process, &program, imports);
    let rng = &mut StdRng::from_entropy();

    println!("Creating deployment");
    // Generate the deployment
    let deployment = process
        .deploy::<A, _>(&program, rng)
        .context("Error process deploy")?;

    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<A::Network, ConsensusMemory<A::Network>>::open(None)
        .context("Error ConsensusStore")?;
    let vm = VM::from(store).context("Error VM")?;

    let fee_authorization: Authorization<A::Network> =
        Authorization::<A::Network>::from_json(fee_authorization_str)
            .context("fee authorization error")?;

    let fee = vm.execute_fee_authorization(
        Authorization_VM::<A::Network>::from(fee_authorization),
        Some(query),
        rng,
    )?;

    // Construct the owner.
    let owner = ProgramOwner::<A::Network>::from_str(owner_str).context("Error ProgramOwner")?;

    // Create a new transaction.
    let transaction =
        Transaction::from_deployment(owner, deployment, fee).context("Error from_deployment")?;

    Ok(transaction.to_string())
}

fn reorder(json_str: &str) -> String {
    let mut json_obj: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");
    let mut json_obj_clone: Value = json_obj.clone();

    let requests = json_obj_clone["requests"].as_array_mut().unwrap();
    let transitions = json_obj["transitions"].as_array_mut().unwrap();
    let mut new_transitions: Vec<Value> = Vec::new();

    for request in requests {
        let tcm = request["tcm"].as_str().unwrap();
        for transition in &mut *transitions {
            if tcm == transition["tcm"].as_str().unwrap() {
                new_transitions.push(transition.clone());
            }
        }
    }

    json_obj["transitions"] = Value::Array(new_transitions);
    serde_json::to_string_pretty(&json_obj).unwrap()
}
