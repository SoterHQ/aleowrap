use snarkvm_console::{
    account::{PrivateKey, ViewKey},
    program::{Address, Environment, FromBytes, Network, ToBytes},
};

use snarkvm_circuit::prelude::PrimeField;
use std::str::FromStr;

use anyhow::{Context, Result};

pub fn is_address<N: Network>(address: &str) -> Result<()> {
    let _ = Address::<N>::from_str(address).context("Error Address try_from")?;
    Ok(())
}

pub fn to_address<N: Network>(private_key: &str) -> Result<String> {
    let private_key =
        PrivateKey::<N>::from_str(private_key).context("Error PrivateKey from_str")?;
    let address = Address::<N>::try_from(private_key).context("Error Address try_from")?;
    Ok(address.to_string())
}

pub fn to_viewkey<N: Network>(private_key: &str) -> Result<String> {
    let private_key =
        PrivateKey::<N>::from_str(private_key).context("Error to_viewkey PrivateKey from_str")?;
    let view_key =
        ViewKey::<N>::try_from(private_key).context("Error to_viewkey ViewKey try_from")?;
    Ok(view_key.to_string())
}

pub fn private_key_from_seed<N: Network>(seed: &[u8]) -> Result<String> {
    let seed: [u8; 32] = seed
        .try_into()
        .context("Error private_key_from_seed seed try_into")?;
    let field = <N as Environment>::Field::from_bytes_le_mod_order(&seed);
    let reader = &*field
        .to_bytes_le()
        .context("Error private_key_from_seed field to_bytes_le")?;
    let seed =
        FromBytes::read_le(reader).context("Error private_key_from_seed FromBytes read_le")?;
    let private_key = PrivateKey::<N>::try_from(seed)
        .context("Error private_key_from_seed PrivateKey try_from")?;
    Ok(private_key.to_string())
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, Rng, SeedableRng};
    use snarkvm_console::{account::PrivateKey, network::MainnetV0};

    use crate::commands::{private_key_from_seed, to_viewkey};

    use super::to_address;

    #[test]
    fn test_to_address() {
        let private_key = PrivateKey::<MainnetV0>::new(&mut StdRng::from_entropy()).unwrap();

        let private_key = private_key.to_string();

        let address = to_address::<MainnetV0>(&private_key).unwrap();

        println!("private_key: {private_key};\naddress: {address}");
    }

    #[test]
    fn test_to_viewkey() {
        let private_key = PrivateKey::<MainnetV0>::new(&mut StdRng::from_entropy()).unwrap();

        let private_key = private_key.to_string();

        let view_key = to_viewkey::<MainnetV0>(&private_key).unwrap();

        println!("private_key: {private_key};\nview_key: {view_key}");
    }

    #[test]
    fn test_private_key_from_seed() {
        let seed: [u8; 32] = StdRng::from_entropy().gen();
        let private_key = private_key_from_seed::<MainnetV0>(&seed).unwrap();
        let address = to_address::<MainnetV0>(&private_key).unwrap();
        let view_key = to_viewkey::<MainnetV0>(&private_key).unwrap();
        println!("\tprivate_key: {private_key}\n\tview_key: {view_key}\n\taddress: {address}");
    }
}
