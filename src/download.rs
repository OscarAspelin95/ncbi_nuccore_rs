use crate::errors::AppError;
use crate::utils::get_url;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

/// The problem with the NCBI response is that it will return a 200 status even
/// if an invalid accession is provided. We should have a regex check first
/// to reduce these kinds of errors. Anyways, we need to check that the
/// response is actually (probably) valid FASTA format.
async fn download_file(url: &str, file_path: &str) -> Result<(), AppError> {
    let response = reqwest::get(url).await?;

    let bytes = match response.error_for_status() {
        Ok(response) => response.bytes().await?,
        Err(e) => {
            return Err(AppError::StatusCodeError(format!(
                "Failed to download file from {}. [ERROR]: `{}`",
                url,
                e.to_string()
            )));
        }
    };

    if !bytes.starts_with(b">") {
        return Err(AppError::InvalidResponseError(format!(
            "{}",
            String::from_utf8_lossy(&bytes)
        )));
    }

    let mut fh = File::create(file_path)?;
    fh.write_all(&bytes)?;
    Ok(())
}

pub async fn download_files(accessions: Vec<String>, outdir: &Path) -> Result<(), AppError> {
    // Here, we actually want a filter map, where we match against an NCBI regex.
    let accession_set: HashSet<String> = accessions
        .iter()
        .map(|accession| accession.trim().to_ascii_uppercase())
        .collect();

    let num_accessions = accession_set.len();

    let bar = ProgressBar::new(accession_set.len() as u64).with_message(format!(
        "Downloading {} unique accesssions",
        &accession_set.len()
    ));
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    bar.enable_steady_tick(Duration::from_millis(200));

    for accession in accession_set {
        let url = get_url(&accession);
        let file_path = format!("{}/{}.fasta", outdir.display(), accession);

        bar.set_message(format!("{}...", accession));

        match download_file(&url, &file_path).await {
            Ok(()) => {
                bar.inc(1);
                bar.set_message(format!("[SUCCESS] {}...", accession));
            }
            Err(e) => {
                bar.set_message(format!(
                    "[FAILED] {}. [ERROR]: {}",
                    accession,
                    e.to_string()
                ));
            }
        }
    }

    bar.abandon_with_message(format!(
        "Downloaded {} out of {} accessions",
        bar.position(),
        num_accessions
    ));

    Ok(())
}
