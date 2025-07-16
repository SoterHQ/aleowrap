use rayon::prelude::*;
use snarkvm_circuit::Aleo;
use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::{Address, Ciphertext, Field, Identifier, Network, Plaintext, ProgramID, Record},
};
use std::str::FromStr;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

// type RecordPlaintext = Record<CurrentNetwork, Plaintext<CurrentNetwork>>;
// type RecordCiphertext = Record<CurrentNetwork, Ciphertext<CurrentNetwork>>;

#[derive(Serialize)]
pub struct RecordData<N: Network> {
    pub record: Record<N, Plaintext<N>>,
    pub serial_number: String,
    #[serde(flatten)]
    pub record_meta: RecordMeta,
}

#[derive(Deserialize)]
pub struct RecordOrgData {
    #[serde(flatten)]
    pub record_meta: RecordMeta,
}

#[derive(Serialize, Deserialize)]
pub struct RecordMeta {
    pub record_ciphertext: String,
    pub identifier: String,
    pub program_id: String,
    pub height: u32,
    pub timestamp: i64,
    pub block_hash: String,
    pub transaction_id: String,
    pub transition_id: String,
    pub function_name: String,
    pub output_index: u8,
    pub input: Option<Vec<String>>,
    #[serde(default)]
    pub address: String,
}

pub fn decrypt_records<A: Aleo>(
    view_key: &str,
    records_orgdata: Vec<String>,
) -> Result<Vec<String>> {
    let view_key = ViewKey::<A::Network>::from_str(view_key)
        .context("[decrypt_records] Error PrivateKey from_str")?;
    let address = Address::<A::Network>::try_from(view_key).context("Error Address try_from")?;
    let address = address.to_string();

    let decrypted_records = records_orgdata
        .par_iter()
        .map(|record| decrypt_record_data(view_key, record, &address))
        .collect::<Result<Vec<String>, _>>()?;

    Ok(decrypted_records)
}

pub fn decrypt_records_with_private_key<N: Network>(
    private_key: &str,
    records_orgdata: Vec<String>,
) -> Result<Vec<String>> {
    let private_key = PrivateKey::<N>::from_str(private_key)
        .context("[decrypt_records] Error PrivateKey from_str")?;

    let address = Address::<N>::try_from(private_key).context("Error Address try_from")?;
    let address = address.to_string();

    let decrypted_records = records_orgdata
        .par_iter()
        .map(|record| decrypt_record_data_with_private_key(private_key, record, &address))
        .collect::<Result<Vec<String>, _>>()?;

    Ok(decrypted_records)
}

fn serial_number_string<N: Network>(
    record: Record<N, Plaintext<N>>,
    private_key: &PrivateKey<N>,
    program_id: &str,
    record_name: &str,
) -> Result<String, String> {
    let view_key = ViewKey::<N>::try_from(private_key)
        .map_err(|_| "View key derivation failed".to_string())?;

    // Compute the record view key.
    let record_view_key = (*record.nonce() * *view_key).to_x_coordinate();

    let commitment = Field::from(
        record
            .to_commitment(
                &ProgramID::from_str(program_id)
                    .map_err(|_| format!("{program_id} is an invalid program name"))?,
                &Identifier::from_str(record_name)
                    .map_err(|_| format!("{record_name} is an invalid identifier"))?,
                &record_view_key,
            )
            .map_err(|e| e.to_string())?,
    );

    let serial_number = Record::<N, Plaintext<N>>::serial_number(*private_key, commitment)
        .map_err(|_| "Serial number derivation failed".to_string())?;
    Ok(serial_number.to_string())
}

pub fn decrypt_record_data<N: Network>(
    view_key: ViewKey<N>,
    record: &str,
    address: &str,
) -> Result<String> {
    let record_org_data: RecordOrgData = serde_json::from_str(record)?;
    if &record_org_data.record_meta.address != "" && record_org_data.record_meta.address != address
    {
        return Ok("".to_string());
    }

    let record =
        Record::<N, Ciphertext<N>>::from_str(&record_org_data.record_meta.record_ciphertext)
            .context("Error RecordCiphertext from_str")?;

    if let Ok(plaintext) = record
        .decrypt(
            &ViewKey::<N>::try_from(view_key)
                .context("[decrypt_records] Error ViewKey try_from")?,
        )
        .context("[decrypt_records] Error record decrypt")
    {
        // let serial_number = serial_number_string(
        //     plaintext.clone(),
        //     &private_key,
        //     &record_org_data.record_meta.program_id,
        //     &record_org_data.record_meta.identifier,
        // )
        // .unwrap_or_default();

        let record_data = RecordData {
            record: plaintext,
            serial_number: "".to_string(),
            record_meta: record_org_data.record_meta,
        };

        return Ok(serde_json::to_string(&record_data)?);
    }

    Ok("".to_string())
}

pub fn decrypt_record_data_with_private_key<N: Network>(
    private_key: PrivateKey<N>,
    record: &str,
    address: &str,
) -> Result<String> {
    let record_org_data: RecordOrgData = serde_json::from_str(record)?;
    if &record_org_data.record_meta.address != "" && record_org_data.record_meta.address != address
    {
        return Ok("".to_string());
    }

    let record =
        Record::<N, Ciphertext<N>>::from_str(&record_org_data.record_meta.record_ciphertext)
            .context("Error RecordCiphertext from_str")?;

    if let Ok(plaintext) = record
        .decrypt(
            &ViewKey::<N>::try_from(private_key)
                .context("[decrypt_records] Error ViewKey try_from")?,
        )
        .context("[decrypt_records] Error record decrypt")
    {
        let serial_number = serial_number_string(
            plaintext.clone(),
            &private_key,
            &record_org_data.record_meta.program_id,
            &record_org_data.record_meta.identifier,
        )
        .unwrap_or_default();

        let record_data = RecordData {
            record: plaintext,
            serial_number,
            record_meta: record_org_data.record_meta,
        };

        return Ok(serde_json::to_string(&record_data)?);
    }

    Ok("".to_string())
}

pub fn decrypt_record<N: Network>(private_key: &str, record: &str) -> Result<String> {
    let record =
        Record::<N, Ciphertext<N>>::from_str(record).context("Error RecordCiphertext try_from")?;
    let private_key = PrivateKey::<N>::from_str(private_key)
        .context("[decrypt_records] Error PrivateKey from_str")?;
    let plaintext = record
        .decrypt(
            &ViewKey::<N>::try_from(private_key)
                .context("[decrypt_records] Error ViewKey try_from")?,
        )
        .context("[decrypt_records] Error record decrypt")?;
    Ok(plaintext.to_string())
}

#[cfg(test)]
mod tests {
    use snarkvm_console::network::MainnetV0;

    use super::*;

    #[test]
    fn test_decrypt_records() {
        let private_key = "APrivateKey1zkpJkyYRGYtkeHDaFfwsKtUJzia7csiWhfBWPXWhXJzy9Ls";
        let mut records_orgdata = Vec::new();
        records_orgdata.push(
            r#"{"record_ciphertext":"record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q","program_id":"aleoswap06.aleo","height":425004,"timestamp":1700055612,"block_hash":"ab1anh0ua3fc08slp39r9qrhfp3x8m0q5cv2wsv26euvh44f7w2cqxqmgteav","transaction_id":"at1nap3det0jpk2kvah2p48fnt0z60lqdk7pj86p4f39na5ne9vgc9s569e84","transition_id":"au16qzgrxm8gsuy0ggcnq92sj8mf6sr8m85nnp27x77k4tdsnxv35qsfhj45z","function_name":"transfer_to_private","output_index":0,"input":null,"identifier":"PrivateToken","address":"aleo1j7qxyunfldj2lp8hsvy7mw5k8zaqgjfyr72x2gh3x4ewgae8v5gscf5jh3"}"#.to_string());
        records_orgdata.push(
            r#"{"record_ciphertext":"record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q","program_id":"aleoswap06.aleo","height":425005,"timestamp":1700055612,"block_hash":"ab1anh0ua3fc08slp39r9qrhfp3x8m0q5cv2wsv26euvh44f7w2cqxqmgteav","transaction_id":"at1nap3det0jpk2kvah2p48fnt0z60lqdk7pj86p4f39na5ne9vgc9s569e84","transition_id":"au16qzgrxm8gsuy0ggcnq92sj8mf6sr8m85nnp27x77k4tdsnxv35qsfhj45z","function_name":"transfer_to_private","output_index":0,"input":null,"identifier":"PrivateToken"}"#.to_string());
        records_orgdata.push(
                r#"{"record_ciphertext":"record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q","program_id":"aleoswap06.aleo","height":425004,"timestamp":1700055612,"block_hash":"ab1anh0ua3fc08slp39r9qrhfp3x8m0q5cv2wsv26euvh44f7w2cqxqmgteav","transaction_id":"at1nap3det0jpk2kvah2p48fnt0z60lqdk7pj86p4f39na5ne9vgc9s569e84","transition_id":"au16qzgrxm8gsuy0ggcnq92sj8mf6sr8m85nnp27x77k4tdsnxv35qsfhj45z","function_name":"transfer_to_private","output_index":0,"input":null,"identifier":"PrivateToken"}"#.to_string());
        records_orgdata.push(
                    r#"{"record_ciphertext":"record1qyqspasur7r5fmazgeu8j0syd82x2p8e66vempwsdepgcjuz8mqwn9ssqyxx66trwfhkxun9v35hguerqqpqzqzj4qnlagqqhr2jnehymmg7ve20gdqaqenrgrf38zh2zycsrnuwp9erjh6elpyd27vjlar4k70ulzcyhhxazlt7jqs82em2vf57pmmqjzqar9n","program_id":"aleoswap06.aleo","height":425005,"timestamp":1700055612,"block_hash":"ab1anh0ua3fc08slp39r9qrhfp3x8m0q5cv2wsv26euvh44f7w2cqxqmgteav","transaction_id":"at1nap3det0jpk2kvah2p48fnt0z60lqdk7pj86p4f39na5ne9vgc9s569e84","transition_id":"au16qzgrxm8gsuy0ggcnq92sj8mf6sr8m85nnp27x77k4tdsnxv35qsfhj45z","function_name":"transfer_to_private","output_index":0,"input":null,"identifier":"PrivateToken"}"#.to_string());
        let records =
            decrypt_records_with_private_key::<MainnetV0>(private_key, records_orgdata).unwrap();
        for record in records {
            println!("record: {record}");
        }
    }

    #[test]
    fn test_decrypt_record_data() {
        let private_key = "APrivateKey1zkpJkyYRGYtkeHDaFfwsKtUJzia7csiWhfBWPXWhXJzy9Ls";
        let record = "record1qyqsqpe2szk2wwwq56akkwx586hkndl3r8vzdwve32lm7elvphh37rsyqyxx66trwfhkxun9v35hguerqqpqzqrtjzeu6vah9x2me2exkgege824sd8x2379scspmrmtvczs0d93qttl7y92ga0k0rsexu409hu3vlehe3yxjhmey3frh2z5pxm5cmxsv4un97q";
        if let Ok(record) = decrypt_record::<MainnetV0>(private_key, record) {
            println!("record: {record}");
        }
    }

    #[test]
    fn test_decrypt_record() {
        // let private_key = "APrivateKey1zkpHvQe27pZV91WMHDhKTVpzcU3jqoyqVCc7iPM7HzsUgjQ";
        // let record = r#"
        // {
        //     "record_ciphertext": "record1qyqsp8httfd678ztpu92nxqenvqdgj3qspl0hppv9qr29ggjtdqglng0qyxx66trwfhkxun9v35hguerqqpqzqz4lyucm3d4aztjk080fg6j2zvd0gqechlxkl6t5n3pfwrgcmh2pd9dxkmu7htj3mzhfym52t0ftavqj5yyhpf83yuct2g03prvqygqky07vuu",
        //     "program_id": "credits.aleo",
        //     "height": 13043,
        //     "timestamp": 1703152900,
        //     "block_hash": "ab16008ac8qqq77npy6vwyk53qp5csvedrclml8rjsk2pxhuftspsqqsng0lq",
        //     "transaction_id": "at1p98x8t80qvl9jwqeymsp07xl689guympmvarc47mexyydkd3avqqwxssdt",
        //     "transition_id": "au10e48x9z40yfp30k2u3dvux4q74kzcetd56mj4wlg69c6e9dnuyqstkflqj",
        //     "function_name": "transfer_public_to_private",
        //     "output_index": 0,
        //     "input": null,
        //     "identifier": "credits",
        //     "address": ""
        //   }
        // "#;
        // let private_key = PrivateKey::<CurrentNetwork>::from_str(private_key).unwrap();
        // let address = AddressNative::try_from(private_key)
        //     .context("Error Address try_from")
        //     .unwrap();
        // let address = address.to_string();
        // let record = decrypt_record_data(private_key, record, &address).unwrap();
        // println!("record: {record}");
    }
}
