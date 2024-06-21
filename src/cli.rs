use structopt::StructOpt;

/// Fields loaded from the command line when launching this server.
#[derive(Clone, Debug, StructOpt)]
#[structopt(name = "Opacity AVS CLI")]
pub struct CliFields {
    /// Configuration file location
    #[structopt(long, default_value = "./config/opacity.config.yaml")]
    pub config_file: String,
    #[structopt(long, default_value = "")]
    pub password: String,
}
