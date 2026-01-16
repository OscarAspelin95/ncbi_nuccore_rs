use indicatif::{ProgressBar, ProgressStyle};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use std::path::Path;
use std::time::Duration;

use crate::errors::AppError;
use std::collections::HashSet;

pub fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_url(accession: &str) -> String {
    format!(
        "https://www.ncbi.nlm.nih.gov/sviewer/viewer.fcgi?id={}&db=nuccore&report=fasta&retmode=text",
        accession
    )
}

pub fn get_progress_bar(length: u64) -> ProgressBar {
    let bar =
        ProgressBar::new(length).with_message(format!("Downloading {} unique accesssions", length));
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    bar.enable_steady_tick(Duration::from_millis(200));

    bar
}

/// Here, we want to add NCBI accession regex validation.
pub fn accession_norm_filt(accessions: Vec<String>) -> Result<HashSet<String>, AppError> {
    let normfilt: HashSet<String> = accessions
        .iter()
        .map(|accession| accession.trim().to_ascii_uppercase())
        .collect();

    match normfilt.is_empty() {
        true => Err(AppError::EmptyAccessionList),
        false => Ok(normfilt),
    }
}

pub fn get_client() -> Result<ClientWithMiddleware, AppError> {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build();

    Ok(client)
}
