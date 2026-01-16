mod args;
mod download;
mod errors;
mod utils;

use args::App;
use clap::Parser;
use download::download_files;
use errors::AppError;
use utils::ensure_dir;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = App::parse();
    ensure_dir(&args.outdir)?;

    download_files(args.accession, &args.outdir).await?;

    Ok(())
}
