use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::{Ciphertext, Field, Identifier, Network, Plaintext, ProgramID, Record},
};
use std::str::FromStr;

use super::CurrentNetwork;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

type RecordPlaintext = Record<CurrentNetwork, Plaintext<CurrentNetwork>>;
type RecordCiphertext = Record<CurrentNetwork, Ciphertext<CurrentNetwork>>;

#[derive(Serialize)]
pub struct OldRecordData<N: Network> {
    record: RecordPlaintext,
    transactionid: N::TransitionID,
    serial_number: Field<CurrentNetwork>,
}

#[derive(Serialize)]
pub struct RecordData {
    record: RecordPlaintext,
    identifier: String,
    serial_number: String,
    program_id: String,
    height: u32,
    timestamp: i64,
    block_hash: String,
    transaction_id: String,
    transition_id: String,
    function_name: String,
    output_index: u8,
    input: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct RecordOrgData {
    record_ciphertext: String,
    identifier: String,
    program_id: String,
    height: u32,
    timestamp: i64,
    block_hash: String,
    transaction_id: String,
    transition_id: String,
    function_name: String,
    output_index: u8,
    input: Option<Vec<String>>,
}

pub fn decrypt_records(private_key: &str, records_orgdata: &str) -> Result<String> {
    let record_org_datas: Vec<RecordOrgData> =
        serde_json::from_str(records_orgdata).unwrap_or_default();
    let mut records = Vec::new();
    for record_org in record_org_datas {
        if let Ok(record) = RecordCiphertext::from_str(&record_org.record_ciphertext) {
            let private_key = PrivateKey::<CurrentNetwork>::from_str(private_key)
                .context("Error PrivateKey from_str")?;
            if let Ok(plaintext) = record.decrypt(
                &ViewKey::<CurrentNetwork>::try_from(private_key)
                    .context("Error ViewKey try_from")?,
            ) {
                let program_id = record_org.program_id.clone();

                let record_name = &record_org.identifier;
                if let Ok(serial_number) =
                    serial_number_string(plaintext.clone(), &private_key, &program_id, record_name)
                {
                    let record_data: RecordData = RecordData {
                        record: plaintext.clone(),
                        identifier: record_org.identifier,
                        serial_number,
                        program_id,
                        height: record_org.height,
                        timestamp: record_org.timestamp,
                        block_hash: record_org.block_hash,
                        transaction_id: record_org.transaction_id,
                        transition_id: record_org.transition_id,
                        function_name: record_org.function_name,
                        output_index: record_org.output_index,
                        input: record_org.input,
                    };
                    records.push(record_data)
                };
            };
        };
    }
    Ok(serde_json::to_string_pretty(&records)
        .unwrap_or_default()
        .replace("\\n", ""))
}

fn serial_number_string(
    record: RecordPlaintext,
    private_key: &PrivateKey<CurrentNetwork>,
    program_id: &str,
    record_name: &str,
) -> Result<String, String> {
    let commitment = Field::from(
        record
            .to_commitment(
                &ProgramID::from_str(program_id)
                    .map_err(|_| format!("{program_id} is an invalid program name"))?,
                &Identifier::from_str(record_name)
                    .map_err(|_| format!("{record_name} is an invalid identifier"))?,
            )
            .map_err(|e| e.to_string())?,
    );

    let serial_number = RecordPlaintext::serial_number(*private_key, commitment.into())
        .map_err(|_| "Serial number derivation failed".to_string())?;
    Ok(serial_number.to_string())
}

pub fn decrypt_record(private_key: &str, record: &str) -> Result<String> {
    let record = RecordCiphertext::from_str(record).context("Error ViewKey try_from")?;
    let private_key = PrivateKey::<CurrentNetwork>::from_str(private_key)
        .context("[decrypt_records] Error PrivateKey from_str")?;
    let plaintext = record
        .decrypt(
            &ViewKey::<CurrentNetwork>::try_from(private_key)
                .context("[decrypt_records] Error ViewKey try_from")?,
        )
        .context("[decrypt_records] Error PrivateKey from_str")?;
    Ok(plaintext.to_string())
}

#[cfg(test)]
mod tests {
    use super::{decrypt_record, decrypt_records};

    #[test]
    fn test_decrypt_records() {
        let private_key = "APrivateKey1zkpJkyYRGYtkeHDaFfwsKtUJzia7csiWhfBWPXWhXJzy9Ls";
        let records_orgdata = "[{\"record_ciphertext\":\"record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q\",\"program_id\":\"credits.aleo\",\"height\":380782,\"timestamp\":1699849707,\"block_hash\":\"ab1v3jzu7htha3mvccd8dazl99nz9lsl8yreq9hqutu3dwsedsfguqsmnfqms\",\"transaction_id\":\"at1ppz4s9x3yc0qata4vu8vucdrehggukejtzxv2ctaqcces5w7tcrqs6tyav\",\"transition_id\":\"au187jyp6xstgyxn0cylnhh4rxa04yekmwp87n9jfrdh7dxp49q3vpq6yklq4\",\"function_name\":\"transfer_public_to_private\",\"output_index\":0,\"input\":null,\"identifier\":\"credits\"}]";
        if let Ok(records) = decrypt_records(private_key, records_orgdata) {
            println!("records: {records}");
        }
    }

    #[test]
    fn test_decrypt_record() {
        let private_key = "APrivateKey1zkpJkyYRGYtkeHDaFfwsKtUJzia7csiWhfBWPXWhXJzy9Ls";
        let record = "record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q";
        if let Ok(record) = decrypt_record(private_key, record) {
            println!("record: {record}");
        }
    }
}
