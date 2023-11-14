use super::{Command, CurrentNetwork};

use snarkvm::prelude::{
    query::Query,
    store::{helpers::memory::ConsensusMemory, ConsensusStore},
    PrivateKey, Value, VM,
};

use anyhow::{Context, Result};
use std::str::FromStr;

pub fn transfer(
    private_key: &str,
    recipient: &str,
    amount: u64,
    function: &str,
    input_record: Option<&str>,
    fee_record: Option<&str>,
    priority_fee_in_microcredits: Option<u64>,
    query: Option<&str>,
) -> Result<String> {
    let query = match query {
        Some(query) => query,
        None => "https://api.explorer.aleo.org/v1",
    };

    // Specify the query
    let query = Query::from(query);

    // Retrieve the private key.
    let private_key = PrivateKey::from_str(private_key).context("private_key is error")?;

    println!(
        "📦 Creating transfer of {} microcredits to {}...\n",
        amount, recipient
    );

    // Prepare the fees.
    let fee_record = match fee_record {
        Some(fee_record) => {
            Some(Command::parse_record(&private_key, fee_record).context("fee_record is error")?)
        },
        None => {
            None
        },
    };
        
    let priority_fee_in_microcredits = priority_fee_in_microcredits.unwrap_or(0);

    // Prepare the inputs for a transfer.

    let (inputs, function) = match function {
        "private" => {
            if input_record.is_none() {
                return Err(anyhow::anyhow!("call transfer_private, input_record is none"));
            }
            let input_record = Command::parse_record(&private_key, input_record.unwrap())
                .context("input_record is error")?;
            (
                vec![
                    Value::Record(input_record),
                    Value::from_str(&format!("{}", recipient)).context("recipient is error")?,
                    Value::from_str(&format!("{}u64", amount)).context("amount is error")?,
                ],
                "transfer_private",
            )
        }
        "public" => (
            vec![
                Value::from_str(&format!("{}", recipient)).context("recipient is error")?,
                Value::from_str(&format!("{}u64", amount)).context("amount is error")?,
            ],
            "transfer_public",
        ),
        "private_to_public" => {
            if input_record.is_none() {
                return Err(anyhow::anyhow!("call transfer_private_to_public,input_record is none"));
            }
            let input_record = Command::parse_record(&private_key, input_record.unwrap())
                .context("input_record is error")?;
            (
                vec![
                    Value::Record(input_record),
                    Value::from_str(&format!("{}", recipient)).context("recipient is error")?,
                    Value::from_str(&format!("{}u64", amount)).context("amount is error")?,
                ],
                "transfer_private_to_public",
            )
        }
        "public_to_private" => (
            vec![
                Value::from_str(&format!("{}", recipient)).context("recipient is error")?,
                Value::from_str(&format!("{}u64", amount)).context("amount is error")?,
            ],
            "transfer_public_to_private",
        ),
        &_ => {
            if input_record.is_none() {
                return Err(anyhow::anyhow!("call transfer_private, input_record is none"));
            }
            let input_record = Command::parse_record(&private_key, input_record.unwrap())
                .context("input_record is error")?;
            (
                vec![
                    Value::Record(input_record),
                    Value::from_str(&format!("{}", recipient)).context("recipient is error")?,
                    Value::from_str(&format!("{}u64", amount)).context("amount is error")?,
                ],
                "transfer_private",
            )
        }
    };

    // Generate the transfer_private transaction.
    // Initialize an RNG.
    let rng = &mut rand::thread_rng();

    // Initialize the VM.
    let store = ConsensusStore::<CurrentNetwork, ConsensusMemory<CurrentNetwork>>::open(None)
        .context("ConsensusStore open error")?;
    let vm = VM::from(store)?;

    // Create a new transaction.
    let transaction = vm
        .execute(
            &private_key,
            ("credits.aleo", function),
            inputs.iter(),
            fee_record,
            priority_fee_in_microcredits,
            Some(query),
            rng,
        )
        .context("execute error")?;

    Ok(transaction.to_string())
}
