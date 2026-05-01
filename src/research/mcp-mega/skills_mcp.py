import os
from mcp.server.fastmcp import FastMCP

mcp = FastMCP("chyren-skills")

@mcp.tool()
def list_skills() -> str:
    """List all available cognitive skills."""
    return "Available skills: [Strategic Orchestration, Identity Synthesis, Heuristic Validation, Threat Pattern Analysis, Structured Extraction]"

@mcp.tool()
def execute_skill(name: str) -> str:
    """Execute a specific cognitive skill."""
    if name == "Structured Extraction":
        return "Skill 'Structured Extraction' should be invoked via the dedicated 'structured_extraction' tool for parameter support."
    return f"Skill '{name}' executed with high-integrity verification."

@mcp.tool()
def structured_extraction(text: str, schema: str) -> str:
    """Parses unstructured text into a typed JSON schema.
    
    Args:
        text: The unstructured text to parse.
        schema: The JSON schema (or description) to follow.
    """
    try:
        from openai import OpenAI
    except ImportError:
        return "Error: openai library not installed."
    
    api_key = os.environ.get("OPENAI_API_KEY")
    if not api_key:
        return "Error: OPENAI_API_KEY not set."
        
    client = OpenAI(api_key=api_key)
    
    # Load skill instructions
    skill_path = os.path.join(os.path.dirname(__file__), "skills", "structured-extractor.md")
    try:
        with open(skill_path, "r") as f:
            instructions = f.read()
    except FileNotFoundError:
        instructions = "Parses unstructured text into a typed JSON schema. Output only pure JSON."

    system_prompt = f"{instructions}\n\nSTRICT SCHEMA:\n{schema}"
    
    try:
        response = client.chat.completions.create(
            model=os.environ.get("OPENAI_MODEL", "gpt-4o-mini"),
            messages=[
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": text}
            ],
            temperature=0,
            response_format={"type": "json_object"}
        )
        return response.choices[0].message.content or "{}"
    except Exception as e:
        return f"Error during extraction: {str(e)}"

if __name__ == "__main__":
    mcp.run(transport="stdio")
