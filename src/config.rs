use serde::Deserialize;
use structopt::StructOpt;

#[derive(Clone, Debug, Deserialize, Default, StructOpt)]
#[structopt(name = "Opacity AVS Config Properties")]
pub struct OpacityAVSConfigProperties {
    #[structopt(short)]
    pub production: bool,
    pub opacity_avs_address: String,
    pub avs_directory_address: String,
    // #[structopt(long, default_value = "17000")]
    pub chain_id: u32,
    // #[structopt(long, default_value = "https://ethereum-holesky.publicnode.com")]
    pub eth_rpc_url: String,
    // #[structopt(long, default_value = "./config/opacity.ecdsa.key.json")]
    pub ecdsa_private_key_store_path: String,
    pub bls_private_key_store_path: String,
}
