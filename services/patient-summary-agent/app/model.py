import logging
from transformers import pipeline  # type: ignore
from config import MODEL_NAME, DEVICE


logging.basicConfig(level=logging.INFO)
pipe = pipeline("text-generation", model=MODEL_NAME, device=DEVICE)
