pub mod bn254;
pub mod cli;
pub mod config;
pub mod errors;
pub mod util;
use ark_bn254::{Bn254, Fq, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{pairing::PairingOutput, AffineRepr, CurveGroup};
use bn254::vec_to_fr;
use cli::CliFields;
use ethers_signers::{LocalWallet, Signer};
use structopt::StructOpt;
use tracing::debug;

fn main() {
    // Load command line arguments which contains the config file location
    let cli_fields: CliFields = CliFields::from_args();
    // Load the configuration file
    let operator_config =
        util::parse_config_file::<config::OpacityAVSConfigProperties>(&cli_fields.config_file)
            .expect("Could not open file or read the file's values.");

    println!("{:?}", operator_config.clone());
    debug!(?operator_config, "Operator config loaded");

    let mut ecdsa_wallet = LocalWallet::decrypt_keystore(
        &operator_config.ecdsa_private_key_store_path,
        &cli_fields.password,
    )
    .unwrap();

    ecdsa_wallet = ecdsa_wallet.with_chain_id(operator_config.chain_id).clone();

    let bn254_private_key = eth_bn254_keystore::decrypt_key(
        &operator_config.bls_private_key_store_path,
        &cli_fields.password,
    )
    .unwrap();

    let bn254_private_key = vec_to_fr(bn254_private_key).unwrap();

    let bn254_public_key_g1 = (G1Affine::generator() * bn254_private_key).to_string();
    println!("BLS Public Key G1: {}", bn254_public_key_g1);

    let bn254_public_key_g2 = (G2Affine::generator() * bn254_private_key).to_string();
    println!("\nBLS Public Key G2: {}", bn254_public_key_g2);
}
