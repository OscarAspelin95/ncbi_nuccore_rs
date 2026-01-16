use crate::errors::AppError;
use crate::utils::{accession_norm_filt, get_client, get_progress_bar, get_url};
use console::style;
use reqwest_middleware::ClientWithMiddleware;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// The problem with the NCBI response is that it will return a 200 status even
/// if an invalid accession is provided. We should have a regex check first
/// to reduce these kinds of errors. Anyways, we need to check that the
/// response is actually (probably) valid FASTA format.
async fn download_file(
    client: &ClientWithMiddleware,
    url: &str,
    file_path: &str,
) -> Result<(), AppError> {
    let response = client.get(url).send().await?;

    let bytes = match response.error_for_status() {
        Ok(response) => response.bytes().await?,
        Err(e) => {
            return Err(AppError::StatusCodeError(format!(
                "Failed to download file from {}. [ERROR]: `{}`",
                url, e
            )));
        }
    };

    if !bytes.starts_with(b">") {
        return Err(AppError::InvalidResponseError(
            String::from_utf8_lossy(&bytes[..bytes.len().min(100)])
                .trim()
                .to_string(),
        ));
    }

    let mut fh = File::create(file_path)?;
    fh.write_all(&bytes)?;
    Ok(())
}

pub async fn download_files(accessions: Vec<String>, outdir: &Path) -> Result<(), AppError> {
    let accession_set = accession_norm_filt(accessions)?;

    let num_accessions = accession_set.len();
    let bar = get_progress_bar(num_accessions as u64);

    let client = get_client()?;

    for accession in accession_set {
        let url = get_url(&accession);
        let file_path = format!("{}/{}.fasta", outdir.display(), accession);

        bar.set_message(format!("{}...", accession));

        match download_file(&client, &url, &file_path).await {
            Ok(()) => {
                bar.inc(1);
                bar.println(format!("{} {}", style("[SUCCESS]").green(), accession));
            }
            Err(e) => {
                bar.println(format!(
                    "{} {}. {}: {}",
                    style("[FAILED]").red(),
                    accession,
                    style("ERROR").yellow(),
                    e
                ));
            }
        }
    }
    bar.finish_with_message(format!(
        "Downloaded {} out of {} accessions",
        bar.position(),
        num_accessions
    ));

    Ok(())
}
