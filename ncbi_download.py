from pathlib import Path
import re
import requests

from utils import _ensure_file, try_with_timeout

ACC_PAT = re.compile(r"^([A-Z]{2}_)?([A-Z]{1,6})?\d{5,15}(\.\d+)?$")


def valid_accession(accession: str) -> bool:
    return ACC_PAT.match(accession) is not None


def get_url(accession: str) -> str:
    return f"https://www.ncbi.nlm.nih.gov/sviewer/viewer.fcgi?id={accession}&db=nuccore&report=fasta&retmode=text"


@try_with_timeout(3, 60 * 5)
def download_fasta(url: str, accession: str, outdir: Path) -> Path:
    fasta = outdir / f"{accession}.fasta"

    with requests.get(url, stream=True, allow_redirects=True, timeout=None) as r:
        r.raise_for_status()
        with fasta.open("wb") as fh:
            for chunk in r.iter_content(chunk_size=8192):
                if chunk:
                    fh.write(chunk)

    _ensure_file(fasta)
    return fasta
