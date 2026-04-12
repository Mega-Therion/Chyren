#!/usr/bin/env python3
"""Update proof-pack metrics.csv from a run status CSV."""

from __future__ import annotations

import argparse
import csv
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
import tomllib


CSV_FIELDS = [
    "metric_name",
    "category",
    "value",
    "unit",
    "source_command",
    "run_id",
    "captured_at_utc",
]


@dataclass
class StepResult:
    run_id: str
    step: str
    status: str
    duration_sec: float
    source_command: str


def read_status_rows(path: Path) -> list[StepResult]:
    rows: list[StepResult] = []
    with path.open(newline="", encoding="utf-8") as f:
        reader = csv.DictReader(f)
        for r in reader:
            rows.append(
                StepResult(
                    run_id=r["run_id"],
                    step=r["step"],
                    status=r["status"],
                    duration_sec=float(r.get("duration_sec", 0) or 0),
                    source_command=r.get("source_command", ""),
                )
            )
    return rows


def read_metrics(path: Path) -> list[dict[str, str]]:
    with path.open(newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def write_metrics(path: Path, rows: list[dict[str, str]]) -> None:
    with path.open("w", newline="", encoding="utf-8") as f:
        writer = csv.DictWriter(f, fieldnames=CSV_FIELDS)
        writer.writeheader()
        writer.writerows(rows)


def upsert_metric(rows: list[dict[str, str]], record: dict[str, str]) -> None:
    name = record["metric_name"]
    for idx, row in enumerate(rows):
        if row.get("metric_name") == name:
            rows[idx] = record
            return
    rows.append(record)


def utc_now_iso() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def count_omega_crates(repo_root: Path) -> int:
    cargo = repo_root / "medulla" / "Cargo.toml"
    if not cargo.exists():
        return 0
    data = tomllib.loads(cargo.read_text(encoding="utf-8"))
    members = data.get("workspace", {}).get("members", [])
    return len([m for m in members if isinstance(m, str) and m.startswith("omega-")])


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--status-file", required=True)
    parser.add_argument("--metrics-file", required=True)
    parser.add_argument("--repo-root", required=True)
    args = parser.parse_args()

    status_file = Path(args.status_file)
    metrics_file = Path(args.metrics_file)
    repo_root = Path(args.repo_root)

    status_rows = read_status_rows(status_file)
    metric_rows = read_metrics(metrics_file)
    timestamp = utc_now_iso()

    pass_metric_map = {
        "rust_tests": "rust_tests_passed",
        "python_tests": "python_tests_passed",
        "web_typecheck": "web_typecheck_passed",
        "web_lint": "web_lint_passed",
    }

    for step in status_rows:
        passed = "1" if step.status == "pass" else "0"
        pass_metric_name = pass_metric_map.get(step.step)

        if pass_metric_name:
            upsert_metric(
                metric_rows,
                {
                    "metric_name": pass_metric_name,
                    "category": "verification",
                    "value": passed,
                    "unit": "boolean",
                    "source_command": step.source_command,
                    "run_id": step.run_id,
                    "captured_at_utc": timestamp,
                },
            )

        upsert_metric(
            metric_rows,
            {
                "metric_name": f"{step.step}_duration_s",
                "category": "performance",
                "value": f"{step.duration_sec:.2f}",
                "unit": "seconds",
                "source_command": step.source_command,
                "run_id": step.run_id,
                "captured_at_utc": timestamp,
            },
        )

    upsert_metric(
        metric_rows,
        {
            "metric_name": "rust_workspace_crates",
            "category": "architecture",
            "value": str(count_omega_crates(repo_root)),
            "unit": "count",
            "source_command": "count omega-* entries in medulla/Cargo.toml",
            "run_id": status_rows[0].run_id if status_rows else "unknown",
            "captured_at_utc": timestamp,
        },
    )

    write_metrics(metrics_file, metric_rows)
    print(f"Updated metrics: {metrics_file}")


if __name__ == "__main__":
    main()
