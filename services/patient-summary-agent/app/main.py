from fastapi import FastAPI # type: ignore
from pydantic import BaseModel # type: ignore
from app.model import generate_advice

app = FastAPI()

class PatientSummary(BaseModel):
    summary: dict

@app.post("/advice")
async def get_advice(payload: PatientSummary):
    advice = generate_advice(payload.summary)
    return { "advice": advice }
