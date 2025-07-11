from transformers import pipeline # type: ignore
import json

pipe = pipeline(
    "text-generation",
    model="google/gemma-3n-e4b-it",
    device_map="auto"
)

def create_prompt(summary: dict) -> str:
    return f"""
        You are a medical assistant. Given the following patient summary in JSON format,
        provide a short, clear medical advice in plain English.
        Patient summary:
        {json.dumps(summary, indent=2)}
        Advice:
    """

def generate_advice(summary: dict) -> str:
    prompt = create_prompt(summary)
    output = pipe(prompt, max_new_tokens=256)[0]["generated_text"]
    return output.split("Advice:")[-1].strip()
