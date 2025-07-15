import csv
from itertools import islice
from pathlib import Path


def get_total_lines(file_path: Path) -> int:
    with open(file_path, encoding="utf-8") as f:
        return sum(1 for _ in f) - 1 # exclude header
    
def get_page_from_csv(file_path: Path, page: int, size: int) -> tuple[list[str], list[list[str]]]:
    start = (page - 1) * size
    end = start + size
    with open(file_path, encoding="utf-8") as f:
        reader = csv.reader(f)
        header = next(reader)
        rows = list(islice(reader, start, end))
    return header, rows
