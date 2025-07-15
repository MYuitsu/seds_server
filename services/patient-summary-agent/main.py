import logging
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

@app.on_event("startup")
async def startup():
    global DATA_MAX_SIZE
    DATA_MAX_SIZE = get_total_lines(FILE_PATH)
    logging.info("Warming up first page of SOAP notes...")
    generate_batch_soap_notes(page=1, size=2)

@app.post("/advice")
async def get_advice(payload: PatientSummary):
    advice = generate_advice(payload.summary)
    return { "advice": advice }

@app.get("/notes") # notes?page=1&size=10
async def get_notes(page: int = Query(1, ge=1), size: int = Query(10, ge=1, le=100)):
    notes: list[str] = generate_batch_soap_notes(page, size)
    return { "page": page, "size": size, "total": DATA_MAX_SIZE, "notes": notes }
