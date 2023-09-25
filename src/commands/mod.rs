type CurrentAleo = snarkvm::circuit::AleoV0;
type CurrentNetwork = snarkvm::prelude::Testnet3;

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

use std::{path::PathBuf, str::FromStr};

use anyhow::{bail, ensure, Result};
use snarkvm::{
    package::Package,
    prelude::{Ciphertext, Plaintext, PrivateKey, ProgramID, Record, ViewKey},
    synthesizer::{Process, Program},
};

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
    fn parse_package(
        program_id: ProgramID<CurrentNetwork>,
        path: Option<String>,
    ) -> Result<Package<CurrentNetwork>> {
        // Instantiate a path to the directory containing the manifest file.
        let directory = match path {
            Some(path) => PathBuf::from_str(&path)?,
            None => std::env::current_dir()?,
        };

        // Load the package.
        let package = Package::open(&directory)?;

        ensure!(
            package.program_id() == &program_id,
            "The program name in the package does not match the specified program name"
        );

        // Return the package.
        Ok(package)
    }
}
