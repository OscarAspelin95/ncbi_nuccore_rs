from pathlib import Path

from pydantic import BaseModel, Field
from sh import curl
from utils import _ensure_file, try_with_timeout


class FastaDownload(BaseModel):
    fasta: str = Field(description="S3 URL to downloaded fasta file.")
    correlation_id: str = Field(description="Run ID for pipeline tracking.")


def get_url(accession: str) -> str:
    return f"https://www.ncbi.nlm.nih.gov/sviewer/viewer.fcgi?id={accession}&db=nuccore&report=fasta&retmode=text"


@try_with_timeout(3, 60 * 5)
def download_fasta(url: str, accession: str, outdir: Path) -> Path:
    fasta = outdir / f"{accession}.fasta"
    curl("-L", "-o", fasta, url)

    _ensure_file(fasta)
    return fasta
