import logging
import time
from app.usecases.csv_usecase import get_page_from_csv
from app.prompts.soap_prompt import create_soap_prompt
from config import MAX_TOKENS, FILE_PATH
from app.model import pipe


_cached_pages: dict[tuple[int, int], list[str]] = {}

def generate_batch_soap_notes(page: int, size: int) -> list[str]:
    cache_key = (page, size)
    if cache_key in _cached_pages:
        logging.info("Returning cached results for page %d, size %d", page, size)
        return _cached_pages[cache_key]
    
    _, rows = get_page_from_csv(FILE_PATH, page, size)
    prompts: list[str] = []
    for row in rows:
        for conversation in row:
            prompt = create_soap_prompt(conversation)
            prompts.append(prompt)
    logging.info("Generated %d prompts.", len(prompts))
    start = time.perf_counter()
    outputs = pipe(prompts, max_new_tokens=MAX_TOKENS, batch_size=4)
    duration = time.perf_counter() - start
    if isinstance(outputs[0], dict) and "generated_text" in outputs[0]:
        results = [out["generated_text"] for out in outputs]
    else:
        results = [clean_output(str(out)) for out in outputs]
    logging.info("Generated %d SOAP notes in %.2f seconds", len(results), duration)
    _cached_pages[cache_key] = results
    return results

def clean_output(out: str) -> str:
    return out.split("SOAP Note:")[-1].replace("*", "").replace("\"}]", "").strip()
