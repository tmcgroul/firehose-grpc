#[derive(clap::Parser)]
pub struct Cli {
    /// Subsquid portal endpoint URL
    #[clap(long)]
    pub portal: String,

    /// Rpc api URL of an ethereum node
    #[clap(long)]
    pub rpc: Option<String>,

    /// Number of blocks after which data is considered final
    #[clap(long)]
    pub finality_confirmation: Option<u64>,
}
