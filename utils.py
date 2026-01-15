import logging
import signal
from collections.abc import Callable
from pathlib import Path
from typing import ParamSpec, TypeVar

from errors import TimeOutError

P = ParamSpec("P")
T = TypeVar("T")


def timeout_handler(signum, frame):
    raise TimeOutError


def _ensure_file(f: str | Path) -> Path:
    if not (f_path := Path(f)).is_file():
        logging.error(f"File {f_path} does not exist")
        raise FileNotFoundError(str(f_path))

    return f_path


def _ensure_dir(d: str | Path) -> Path:
    if not (d_path := Path(d)).is_dir():
        d_path.mkdir(parents=True)

    return d_path


def run_with_timeout(timeout: int, func: Callable[P, T], *args: P.args, **kwargs: P.kwargs) -> T:
    """Run a function with a timeout."""
    signal.signal(signal.SIGALRM, timeout_handler)
    signal.alarm(timeout)

    try:
        result = func(*args, **kwargs)
    finally:
        signal.alarm(0)

    return result


def try_with_timeout(num_retries: int, timeout: int):
    """Decorator that allows a function to run with timeout (in seconds)."""

    def wrapper(func: Callable[P, T]) -> Callable[P, T]:
        def inner(*args: P.args, **kwargs: P.kwargs):
            for _ in range(0, num_retries):
                try:
                    return run_with_timeout(timeout, func, *args, **kwargs)
                except TimeOutError:
                    logging.info("Retrying due to timeout...")
                    continue

            raise TimeOutError

        return inner

    return wrapper
