import os
import sys
import json
from openai import OpenAI

def transcribe_audio(file_path):
    api_key = os.environ.get("GROQ_API_KEY")
    if not api_key:
        print("Error: GROQ_API_KEY not set")
        return None

    client = OpenAI(
        base_url="https://api.groq.com/openai/v1",
        api_key=api_key
    )

    print(f"Transcribing {file_path} via Groq Whisper...")
    with open(file_path, "rb") as file:
        transcription = client.audio.transcriptions.create(
            file=(os.path.basename(file_path), file.read()),
            model="whisper-large-v3",
            response_format="text"
        )
    return transcription

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python transcribe.py <file_path>")
        sys.exit(1)
        
    file_path = sys.argv[1]
    text = transcribe_audio(file_path)
    if text:
        output_path = file_path.replace(".mp3", ".md").replace("_compressed", "")
        with open(output_path, "w") as f:
            f.write(f"# Transcription: {os.path.basename(file_path)}\n\n")
            f.write(text)
        print(f"✓ Transcription saved to {output_path}")
    else:
        print("Transcription failed.")
