use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli{
    /// The path to the Solidity contract file to analyze.
    #[arg(required = true)]
    pub contract_path: PathBuf,

    /// The path to write the output report to.
    #[arg(short, long, default_value = "report.md")]
    pub output: PathBuf,
}