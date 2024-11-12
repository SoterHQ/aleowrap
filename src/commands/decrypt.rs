// use snarkvm::{
//     console::program::{Identifier, Parser, ToField},
//     prelude::{Ciphertext, Field, Network, PrivateKey,ComputeKey, ProgramID, Address}, utilities::{ToBits, Uniform},
// };

use snarkvm_circuit::Aleo;
use snarkvm_console::account::{Address, ComputeKey, PrivateKey};
use snarkvm_console::program::{
    Ciphertext, Field, Identifier, Network, Parser, ProgramID, ToField,
};
use snarkvm_utilities::{ToBits, Uniform};

use std::str::FromStr;

use anyhow::{Context, Result};

pub fn decrypt_ciphertext<A: Aleo>(private_key: &str, ciphertext: &str) -> Result<String> {
    let (remainder, ciphertext) = Ciphertext::<A::Network>::parse(ciphertext).unwrap();
    println!("ciphertext: {}", ciphertext.to_string());
    println!("remainder: {remainder}");

    // Construct a network ID.
    let network_id = A::Network::ID;
    // Construct a program ID.
    let program_id = ProgramID::<A::Network>::from_str("credits.aleo")?;
    // Construct a function name.
    let function_name = Identifier::<A::Network>::from_str("transfer_private")?;

    let function_id = A::Network::hash_bhp1024(
        &(
            network_id,
            program_id.name(),
            program_id.network(),
            function_name,
        )
            .to_bits_le(),
    )?;
    // let tvp = "7243276130204134754279324811741016619127503709911162509525331265511397689608group";
    // let tvp = Group::<CurrentAleo>::from_str(tvp).unwrap();

    let private_key =
        PrivateKey::<A::Network>::from_str(private_key).context("Error PrivateKey from_str")?;
    // Derive the compute key.
    let compute_key = ComputeKey::try_from(private_key)?;

    // Derive the signer from the compute key.
    let signer = Address::try_from(compute_key)?;
    // Retrieve `pk_sig`.
    // let pk_sig = compute_key.pk_sig();
    // // Retrieve `pr_sig`.
    // let pr_sig = compute_key.pr_sig();

    // Retrieve `sk_sig`.
    let sk_sig = private_key.sk_sig();

    // Derive the view key.
    // let view_key = ViewKey::<CurrentNetwork>::try_from(private_key)?;
    // Derive `sk_tag` from the graph key.
    // let sk_tag = GraphKey::try_from(view_key)?.sk_tag();

    // // Initialize an RNG.
    let rng = &mut rand::thread_rng();
    // Sample a random nonce.
    let nonce = Field::<A::Network>::rand(rng);
    // Compute a `r` as `HashToScalar(sk_sig || nonce)`. Note: This is the transition secret key `tsk`.
    let r = A::Network::hash_to_scalar_psd4(&[
        A::Network::serial_number_domain(),
        sk_sig.to_field()?,
        nonce,
    ])?;
    // Compute `g_r` as `r * G`. Note: This is the transition public key `tpk`.
    // let g_r = CurrentNetwork::g_scalar_multiply(&r);

    // Compute the transition view key `tvk` as `r * signer`.
    let tvk = (*signer * r).to_x_coordinate();

    let index = Field::from_u16(1 as u16);

    let plaintext_view_key = A::Network::hash_psd4(&[function_id, tvk, index])?;
    let plaintext = ciphertext.decrypt_symmetric(plaintext_view_key)?;
    println!("plaintext: {}", plaintext.to_string());

    Ok(remainder.to_string())
}

#[cfg(test)]
mod tests {
    use snarkvm_circuit::AleoV0;

    use super::decrypt_ciphertext;

    #[test]
    fn test_decrypt_ciphertext() {
        let plaintext = decrypt_ciphertext::<AleoV0>("APrivateKey1zkp6ZYopKYbJakUtmwgjZ6DAkbvzW592msjZX4Q8SUbk9sN", "ciphertext1qgq9z7ks2dzdwpc7r4ul323u45dyg060na43r4fhfm0ctkmkx4u0xzgpkpnekdskggxnj5fh4yux9sd3ca42nclv7dfr0szx8new6z4hpg6fyhy5");
        println!("plaintext: {}", plaintext.unwrap());
    }
}
