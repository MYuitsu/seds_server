def create_soap_prompt(conversation: str) -> str:
    return f"You are a helpful medical assistant. Based on the following conversation, generate a SOAP note.\n\nConversation:\n{conversation}\n\nSOAP Note:"
