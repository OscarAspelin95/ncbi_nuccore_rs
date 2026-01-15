import argparse
import logging
from pathlib import Path

from ncbi_download import download_fasta, get_url
from utils import _ensure_dir


def main(accessions: list[str], outdir: Path):
    for accession in map(lambda x: x.strip(), accessions):
        url = get_url(accession)

        log.info(f"Downloading {accession}")
        fasta = download_fasta(url, accession, outdir)
        log.info(f"Downloaded {accession} to {fasta}")


if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    log = logging.getLogger(__name__)

    parser = argparse.ArgumentParser(description="Download NCBI FASTA file(s) from nuccore")
    parser.add_argument("-a", "--accession", nargs="+", help="NCBI accession number", required=True)
    parser.add_argument(
        "-o",
        "--outdir",
        help="Output directory",
        type=Path,
        required=False,
        default="./ncbi_nuccore_download",
    )
    args = parser.parse_args()
    outdir = _ensure_dir(args.outdir)

    main(args.accession, outdir)
