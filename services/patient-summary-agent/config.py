from pathlib import Path


# Base directory where data lives
BASE_DIR = Path(__file__).parent

# Path to the test data
FILE_PATH = BASE_DIR / "data/test_llama_formatted.csv"

MODEL_NAME = "google/gemma-1.1-2b-it"
MAX_TOKENS = 128
DEVICE = -1  # CPU (-1), GPU (0), or "auto"
