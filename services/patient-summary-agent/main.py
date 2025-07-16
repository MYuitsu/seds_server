import logging
import threading
import time
from math import ceil
from fastapi import FastAPI, Query # type: ignore
from pydantic import BaseModel # type: ignore
from config import FILE_PATH
from app.usecases.advice_usecase import generate_advice
from app.usecases.csv_usecase import get_total_lines
from app.usecases.soap_notes_usecase import generate_batch_soap_notes


app = FastAPI()
DATA_MAX_SIZE: int
class PatientSummary(BaseModel):
    summary: dict

def generate_all_notes_background(page: int, size: int, total_pages: int):
    current_page = page
    logging.info("Starting background SOAP notes generation...")
    while current_page <= total_pages:
        start_time = time.time()
        logging.info(f"Generating page {current_page}...")
        try:
            generate_batch_soap_notes(page=current_page, size=size)
            logging.info(f"Page {current_page} generated in {time.time() - start_time:.2f} seconds.")
        except Exception as e:
            logging.error(f"Failed to generate page {current_page}: {e}")
        current_page += 1

@app.on_event("startup")
async def startup():
    global DATA_MAX_SIZE
    DATA_MAX_SIZE = get_total_lines(FILE_PATH)
    size = 2
    total_pages = ceil(DATA_MAX_SIZE / size)
    threading.Thread(
        target=generate_all_notes_background, 
        args=(1, size, total_pages), 
        daemon=True
    ).start()

@app.post("/advice")
async def get_advice(payload: PatientSummary):
    advice = generate_advice(payload.summary)
    return { "advice": advice }

@app.get("/notes") # notes?page=1&size=10
async def get_notes(page: int = Query(1, ge=1), size: int = Query(10, ge=1, le=100)):
    notes: list[str] = generate_batch_soap_notes(page, size)
    return { "page": page, "size": size, "total": DATA_MAX_SIZE, "notes": notes }
