---
name: structured-extractor
description: Parses unstructured text into a typed JSON schema. Use when extracting data from emails, PDFs, logs, transcripts, or scraped HTML based on a provided JSON schema.
---

# Structured Extractor

Parses unstructured text into a typed JSON schema.

## Workflow

1. **Provide Schema & Input**: When the user provides unstructured text and a schema, perform extraction.
2. **Read Schema**: Identify required fields, types, and constraints.
3. **Scan & Normalize**: Extract fields, normalizing types (dates to ISO 8601, numeric currency) and handling missing values with `null`.
4. **Emit JSON**: Output only the final JSON object or array. No prose or markdown fences.

## Rules

- **Strict Schema Adherence**: Only include fields defined in the schema.
- **Normalization**: Trim whitespace, coerce formats, collapse enum synonyms.
- **Ambiguity**: Use the `_extraction_notes` field if the schema allows additional properties.
- **Output**: Pure JSON only.
