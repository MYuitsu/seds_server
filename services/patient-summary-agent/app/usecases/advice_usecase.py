import time
import logging
from typing import Any, Dict
from app.prompts.advice_prompt import create_prompt, clean_response
from app.model import pipe
from config import MAX_TOKENS


def generate_advice(summary: Dict[str, Any]) -> str:
    prompt = create_prompt(summary)
    logging.info("Generated prompt:\n%s", prompt)
    start = time.perf_counter()
    result = pipe(
        prompt,
        max_new_tokens=MAX_TOKENS,
        eos_token_id=50256,
        pad_token_id=50256
    )
    duration = time.perf_counter() - start
    logging.info("Took %.2fs", duration)
    generated = clean_response(result[0].get("generated_text", ""))
    logging.info("Generated response:\n%s", generated)
    return generated
