type CurrentAleo = snarkvm_circuit::AleoV0;
type CurrentNetwork = snarkvm_console::network::Testnet3;

mod cost;
pub use cost::*;

mod execute;
pub use execute::*;

mod deploy;
pub use deploy::*;

mod transfer;
pub use transfer::*;

mod join;
pub use join::*;

mod split;
pub use split::*;

mod record;
pub use record::*;

mod account;
pub use account::*;

mod decrypt;
pub use decrypt::*;

use std::{path::PathBuf, str::FromStr};

use anyhow::{bail, ensure, Result};

use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::{Ciphertext, Plaintext, ProgramID, Record},
};
use snarkvm_ledger_block::transaction::Transaction;
use snarkvm_synthesizer::{Process, Program};
use snarkvm_utilities::ToBytes;

pub struct Command {}

impl Command {
    fn parse_record(
        private_key: &PrivateKey<CurrentNetwork>,
        record: &str,
    ) -> Result<Record<CurrentNetwork, Plaintext<CurrentNetwork>>> {
        match record.starts_with("record1") {
            true => {
                // Parse the ciphertext.
                let ciphertext =
                    Record::<CurrentNetwork, Ciphertext<CurrentNetwork>>::from_str(record)?;
                // Derive the view key.
                let view_key: ViewKey<CurrentNetwork> = ViewKey::try_from(private_key)?;
                // Decrypt the ciphertext.
                ciphertext.decrypt(&view_key)
            }
            false => Record::<CurrentNetwork, Plaintext<CurrentNetwork>>::from_str(record),
        }
    }

    /// Fetch the program from the given endpoint.
    fn fetch_program(
        program_id: &ProgramID<CurrentNetwork>,
        endpoint: &str,
    ) -> Result<Program<CurrentNetwork>> {
        // Send a request to the query node.
        let response = ureq::get(&format!("{endpoint}/testnet3/program/{program_id}")).call();

        // Deserialize the program.
        match response {
            Ok(response) => response.into_json().map_err(|err| err.into()),
            Err(err) => match err {
                ureq::Error::Status(_status, response) => {
                    bail!(response
                        .into_string()
                        .unwrap_or("Response too large!".to_owned()))
                }
                err => bail!(err),
            },
        }
    }

    /// A helper function to recursively load the program and all of its imports into the process.
    fn load_program(
        endpoint: &str,
        process: &mut Process<CurrentNetwork>,
        program_id: &ProgramID<CurrentNetwork>,
    ) -> Result<()> {
        // Fetch the program.
        let program = Command::fetch_program(program_id, endpoint)?;

        // Return early if the program is already loaded.
        if process.contains_program(program.id()) {
            return Ok(());
        }

        // Iterate through the program imports.
        for import_program_id in program.imports().keys() {
            // Add the imports to the process if does not exist yet.
            if !process.contains_program(import_program_id) {
                // Recursively load the program and its imports.
                Self::load_program(endpoint, process, import_program_id)?;
            }
        }

        // Add the program to the process if it does not already exist.
        if !process.contains_program(program.id()) {
            process.add_program(&program)?;
        }

        Ok(())
    }

    /// Parse the package from the directory.
    // fn parse_package(
    //     program_id: ProgramID<CurrentNetwork>,
    //     path: Option<String>,
    // ) -> Result<Package<CurrentNetwork>> {
    //     // Instantiate a path to the directory containing the manifest file.
    //     let directory = match path {
    //         Some(path) => PathBuf::from_str(&path)?,
    //         None => std::env::current_dir()?,
    //     };

    //     // Load the package.
    //     let package = Package::open(&directory)?;

    //     ensure!(
    //         package.program_id() == &program_id,
    //         "The program name in the package does not match the specified program name"
    //     );

    //     // Return the package.
    //     Ok(package)
    // }

    /// Determine if the transaction should be broadcast or displayed to user.
    pub fn handle_transaction(
        broadcast: Option<String>,
        dry_run: bool,
        store: Option<String>,
        transaction: Transaction<CurrentNetwork>,
        operation: String,
    ) -> Result<String> {
        // Get the transaction id.
        let transaction_id = transaction.id();

        // Ensure the transaction is not a fee transaction.
        ensure!(
            !transaction.is_fee(),
            "The transaction is a fee transaction and cannot be broadcast"
        );

        // Determine if the transaction should be stored.
        if let Some(path) = store {
            match PathBuf::from_str(&path) {
                Ok(file_path) => {
                    let transaction_bytes = transaction.to_bytes_le()?;
                    std::fs::write(&file_path, transaction_bytes)?;
                    println!(
                        "Transaction {transaction_id} was stored to {}",
                        file_path.display()
                    );
                }
                Err(err) => {
                    println!("The transaction was unable to be stored due to: {err}");
                }
            }
        };

        // Determine if the transaction should be broadcast to the network.
        if let Some(endpoint) = broadcast {
            // Send the deployment request to the local development node.
            match ureq::post(&endpoint).send_json(&transaction) {
                Ok(id) => {
                    // Remove the quotes from the response.
                    let response_string = id.into_string()?.trim_matches('\"').to_string();
                    ensure!(
                        response_string == transaction_id.to_string(),
                        "The response does not match the transaction id. ({response_string} != {transaction_id})"
                    );

                    match transaction {
                        Transaction::Deploy(..) => {
                            println!(
                                "✅ Successfully broadcast deployment {transaction_id} ('{}') to {}.",
                                operation,
                                endpoint
                            )
                        }
                        Transaction::Execute(..) => {
                            println!(
                                "✅ Successfully broadcast execution {transaction_id} ('{}') to {}.",
                                operation,
                                endpoint
                            )
                        }
                        Transaction::Fee(..) => {
                            println!(
                                "❌ Failed to broadcast fee '{}' to the {}.",
                                operation, endpoint
                            )
                        }
                    }
                }
                Err(error) => {
                    let error_message = match error {
                        ureq::Error::Status(code, response) => {
                            format!("(status code {code}: {:?})", response.into_string()?)
                        }
                        ureq::Error::Transport(err) => format!("({err})"),
                    };

                    match transaction {
                        Transaction::Deploy(..) => {
                            bail!(
                                "❌ Failed to deploy '{}' to {}: {}",
                                operation,
                                &endpoint,
                                error_message
                            )
                        }
                        Transaction::Execute(..) => {
                            bail!(
                                "❌ Failed to broadcast execution '{}' to {}: {}",
                                operation,
                                &endpoint,
                                error_message
                            )
                        }
                        Transaction::Fee(..) => {
                            bail!(
                                "❌ Failed to broadcast fee '{}' to {}: {}",
                                operation,
                                &endpoint,
                                error_message
                            )
                        }
                    }
                }
            };

            // Output the transaction id.
            Ok(transaction_id.to_string())
        } else if dry_run {
            // Output the transaction string.
            Ok(transaction.to_string())
        } else {
            Ok("".to_string())
        }
    }
}
