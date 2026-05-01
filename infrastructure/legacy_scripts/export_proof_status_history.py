#!/usr/bin/env python3
"""Export proof status history into one CSV for Google Sheets analysis."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


FIELDS = ["run_id", "step", "status", "duration_sec", "source_command", "status_file"]


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--raw-dir", required=True, help="Directory containing *-status.csv files")
    parser.add_argument("--output", required=True, help="Output CSV path")
    args = parser.parse_args()

    raw_dir = Path(args.raw_dir)
    output = Path(args.output)
    rows: list[dict[str, str]] = []

    for path in sorted(raw_dir.glob("*-status.csv")):
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            for row in reader:
                rows.append(
                    {
                        "run_id": row.get("run_id", ""),
                        "step": row.get("step", ""),
                        "status": row.get("status", ""),
                        "duration_sec": row.get("duration_sec", ""),
                        "source_command": row.get("source_command", ""),
                        "status_file": path.name,
                    }
                )

    output.parent.mkdir(parents=True, exist_ok=True)
    with output.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=FIELDS)
        writer.writeheader()
        writer.writerows(rows)

    print(f"Exported {len(rows)} rows to {output}")


if __name__ == "__main__":
    main()
