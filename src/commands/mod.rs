mod cost;
pub use cost::*;

mod execute;
pub use execute::*;

mod deploy;
pub use deploy::*;

mod transfer;
use snarkvm_ledger_query::Query;
use snarkvm_ledger_store::helpers::memory::BlockMemory;
pub use transfer::*;

mod join;
pub use join::*;

mod split;
pub use split::*;

mod record;
pub use record::*;

mod account;
pub use account::*;

mod authorization;
pub use authorization::*;

mod decrypt;
pub use decrypt::*;

use std::str::FromStr;

use anyhow::Result;

use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::{Ciphertext, Network, Plaintext, ProgramID, Record},
};
use snarkvm_synthesizer::Process;

pub use snarkvm_circuit::{Aleo, AleoCanaryV0, AleoTestnetV0, AleoV0};
pub use snarkvm_console::network::{CanaryV0, MainnetV0, TestnetV0};

pub struct Command {}

impl Command {
    fn parse_record<N: Network>(
        private_key: &PrivateKey<N>,
        record: &str,
    ) -> Result<Record<N, Plaintext<N>>> {
        match record.starts_with("record1") {
            true => {
                // Parse the ciphertext.
                let ciphertext = Record::<N, Ciphertext<N>>::from_str(record)?;
                // Derive the view key.
                let view_key: ViewKey<N> = ViewKey::try_from(private_key)?;
                // Decrypt the ciphertext.
                ciphertext.decrypt(&view_key)
            }
            false => Record::<N, Plaintext<N>>::from_str(record),
        }
    }

    /// A helper function to recursively load the program and all of its imports into the process.
    fn load_program<N: Network>(
        query: &Query<N, BlockMemory<N>>,
        process: &mut Process<N>,
        program_id: &ProgramID<N>,
    ) -> Result<()> {
        // Fetch the program.
        let program = query.get_program(program_id)?;

        // Return early if the program is already loaded.
        if process.contains_program(program.id()) {
            return Ok(());
        }

        // Iterate through the program imports.
        for import_program_id in program.imports().keys() {
            // Add the imports to the process if does not exist yet.
            if !process.contains_program(import_program_id) {
                // Recursively load the program and its imports.
                Self::load_program(query, process, import_program_id)?;
            }
        }

        // Add the program to the process if it does not already exist.
        if !process.contains_program(program.id()) {
            process.add_program(&program)?;
        }

        Ok(())
    }
}
