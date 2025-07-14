import time
import logging
from typing import Any, Dict
from transformers import pipeline  # type: ignore

# --- Config ---
MODEL_NAME = "google/gemma-1.1-2b-it" # "tiiuae/falcon-rw-1b"
MAX_TOKENS = 128
DEVICE = -1  # CPU (-1) or GPU (0) or "auto"

# --- Init ---
logging.basicConfig(level=logging.INFO)
pipe = pipeline("text-generation", model=MODEL_NAME, device=DEVICE)

# --- Prompt Builder ---
def create_prompt(summary: Dict[str, Any]) -> str:
    care_plans = summary.get("inpatientCarePlansRecord", [])
    if not care_plans:
        return (
            "You are a helpful medical assistant. No care plan information is available. "
            "Respond with general healthy living advice.\n\nAdvice:"
        )

    prompt_lines = ["You are a helpful medical assistant. Based on the following care plan:"]
    
    for plan in care_plans:
        issues = ", ".join(plan.get("Addresses", [])) or "None specified"
        goals = plan.get("Goal", [])
        if isinstance(goals, str):
            goals = [goals]
        goals_text = "; ".join(goal for goal in goals if goal != "_") or "None specified"
        prompt_lines.append(f"- Issues: {issues}")
        prompt_lines.append(f"  Goals: {goals_text}")

    prompt_lines.append(
        "\nProvide 2–3 short and practical pieces of plain-English medical advice.\n\nAdvice:"
    )
    return "\n".join(prompt_lines)

# --- Response Cleanup ---
def clean_response(raw_text: str) -> str:
    advice = raw_text.split("Advice:")[-1].strip()
    lines = [line.strip("- ").strip() for line in advice.splitlines() if line.strip()]
    seen, final = set(), []
    for line in lines:
        if line and line not in seen and len(final) < 3:
            seen.add(line)
            final.append(line)
    return "\n".join(f"- {l}" for l in final)

# --- Advice Generator ---
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
