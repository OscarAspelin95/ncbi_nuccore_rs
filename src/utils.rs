use std::path::Path;

pub fn ensure_dir(path: &Path) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_url(accession: &str) -> String {
    return format!(
        "https://www.ncbi.nlm.nih.gov/sviewer/viewer.fcgi?id={}&db=nuccore&report=fasta&retmode=text",
        accession
    );
}
