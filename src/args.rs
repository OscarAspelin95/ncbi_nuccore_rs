use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Download FASTA files from NCBI nuccore."
)]
pub struct App {
    #[arg(short, long, required = true, value_delimiter = ' ', num_args = 1..)]
    pub accession: Vec<String>,

    /// The output file to use
    #[arg(short, long, required = true)]
    pub outdir: PathBuf,
}
