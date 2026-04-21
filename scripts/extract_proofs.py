import json
import glob
import os

files = glob.glob("/home/mega/.claude/projects/-home-mega-Chyren/*.jsonl")
count = 0
for filepath in files:
    with open(filepath, "r") as f:
        for line in f:
            try:
                data = json.loads(line)
                if "message" in data and data["message"].get("role") == "assistant":
                    content_list = data["message"].get("content", [])
                    for content in content_list:
                        if content.get("type") == "text":
                            text = content.get("text", "")
                            # Check if it looks like a Millennium problem proof response
                            if "ESTABLISHED" in text and any(x in text for x in ["Navier-Stokes", "P vs NP", "Riemann", "Hodge", "Poincaré", "Birch"]):
                                count += 1
                                out_path = f"/home/mega/extracted_proof_{count}.md"
                                print(f"Found proof! Writing to {out_path}")
                                with open(out_path, "w") as out:
                                    out.write(text)
            except Exception as e:
                pass
