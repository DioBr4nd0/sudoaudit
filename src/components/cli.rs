use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A smart contract security analysis tool.")]
pub struct Cli {
    /// The path to the Solidity contract file to analyze.
    #[arg(required = true)]
    pub contract_path: std::path::PathBuf,
}