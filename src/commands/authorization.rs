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

use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::CurrentNetwork;
use anyhow::{Context, Result};
use snarkvm_ledger_query::Query;
use snarkvm_ledger_store::{helpers::memory::ConsensusMemory, ConsensusStore};
use snarkvm_synthesizer::Authorization as Authorization_VM;
use snarkvm_synthesizer::VM;

pub type AuthorizationNative = Authorization_VM<CurrentNetwork>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Authorization(AuthorizationNative);

impl Authorization {
    /// Create a authorization from a string
    ///
    /// @param {string} authorization String representation of a authorization
    /// @returns {authorization | Error}
    pub fn from_string(authorization: &str) -> Result<Authorization, String> {
        Authorization::from_str(authorization)
    }

    /// Get the authorization as a string. If you want to submit this authorization to the Aleo Network
    /// this function will create the string that should be submitted in the `POST` data.
    ///
    /// @returns {string} String representation of the authorization
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Authorization> for AuthorizationNative {
    fn from(authorization: Authorization) -> Self {
        authorization.0
    }
}

impl From<AuthorizationNative> for Authorization {
    fn from(authorization: AuthorizationNative) -> Self {
        Self(authorization)
    }
}

impl FromStr for Authorization {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            AuthorizationNative::from_str(s).map_err(|e| e.to_string())?,
        ))
    }
}

pub fn transaction_for_authorize(
    execute_authorization_str: &str,
    fee_authorization_str: &str,
    query: Option<&str>,
) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://api.explorer.aleo.org/v1",
    };

    // Specify the query
    let query = Query::from(query);

    // Generate the transfer_private transaction.
    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // let execute_authorization: Authorization = serde_json::from_str::<execute_authorization_str>.unwrap();
    let execute_authorization: Authorization =
        serde_json::from_str(execute_authorization_str).unwrap();
    let fee_authorization: Authorization = serde_json::from_str(fee_authorization_str).unwrap();

    // let execute_authorization = Authorization::<CurrentNetwork>::from_str(execute_authorization)
    //     .context("execute_authorization open error")?;
    // let fee_authorization = Authorization::<CurrentNetwork>::from_str(fee_authorization)
    //     .context("fee_authorization open error")?;
    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
        .context("ConsensusStore open error")?;
    let vm = VM::from(store)?;

    let transaction = vm
        .execute_authorization(
            AuthorizationNative::from(execute_authorization),
            Some(AuthorizationNative::from(fee_authorization)),
            Some(query),
            rng,
        )
        .context("execute error")?;

    Ok(transaction.to_string())
}
