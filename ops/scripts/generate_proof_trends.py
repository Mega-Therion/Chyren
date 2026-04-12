#!/usr/bin/env python3
"""Generate trend SVGs from proof-pack status CSV history.

Reads all *-status.csv files in a raw history folder and emits:
1) verification-pass-rate-trend.svg
2) step-duration-trend.svg
3) run-stability-heatmap.svg
"""

from __future__ import annotations

import argparse
import csv
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path


@dataclass
class StepRow:
    run_id: str
    step: str
    status: str
    duration_sec: float


def utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%SZ")


def parse_run_timestamp(run_id: str) -> datetime:
    # run_id format: proof-YYYYMMDDTHHMMSSZ
    try:
        token = run_id.removeprefix("proof-")
        return datetime.strptime(token, "%Y%m%dT%H%M%SZ").replace(tzinfo=timezone.utc)
    except ValueError:
        return datetime.min.replace(tzinfo=timezone.utc)


def read_history(raw_dir: Path) -> list[StepRow]:
    rows: list[StepRow] = []
    for path in sorted(raw_dir.glob("*-status.csv")):
        with path.open(newline="", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            for r in reader:
                rows.append(
                    StepRow(
                        run_id=r.get("run_id", ""),
                        step=r.get("step", ""),
                        status=r.get("status", ""),
                        duration_sec=float(r.get("duration_sec", 0) or 0),
                    )
                )
    return rows


def write_svg(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")


def grouped_runs(rows: list[StepRow]) -> list[str]:
    run_ids = sorted({r.run_id for r in rows}, key=parse_run_timestamp)
    return run_ids


def pass_rate_per_run(rows: list[StepRow], run_ids: list[str]) -> list[tuple[str, float]]:
    out: list[tuple[str, float]] = []
    for run_id in run_ids:
        run_rows = [r for r in rows if r.run_id == run_id]
        if not run_rows:
            out.append((run_id, 0.0))
            continue
        passed = sum(1 for r in run_rows if r.status == "pass")
        out.append((run_id, (passed / len(run_rows)) * 100.0))
    return out


def line_chart_svg(
    title: str,
    series: list[tuple[str, list[float]]],
    x_labels: list[str],
    y_max: float,
    width: int = 1180,
    height: int = 580,
) -> str:
    margin = 90
    chart_w = width - 2 * margin
    chart_h = height - 2 * margin
    y_max = max(1.0, y_max)
    palette = ["#4f46e5", "#06b6d4", "#10b981", "#f59e0b", "#ef4444", "#e11d48"]

    def x_pos(i: int) -> float:
        if len(x_labels) <= 1:
            return margin + chart_w / 2
        return margin + (i / (len(x_labels) - 1)) * chart_w

    def y_pos(v: float) -> float:
        return margin + chart_h - (v / y_max) * chart_h

    grid = []
    for t in range(6):
        val = (y_max / 5) * t
        y = y_pos(val)
        grid.append(f'<line x1="{margin}" y1="{y:.1f}" x2="{margin + chart_w}" y2="{y:.1f}" stroke="#243247" stroke-width="1" />')
        grid.append(f'<text x="{margin - 12}" y="{y + 4:.1f}" text-anchor="end" font-size="11" fill="#94a3b8">{val:.0f}</text>')

    x_ticks = []
    for i, label in enumerate(x_labels):
        x = x_pos(i)
        short = label.removeprefix("proof-")
        x_ticks.append(f'<text x="{x:.1f}" y="{margin + chart_h + 24}" text-anchor="middle" font-size="10" fill="#94a3b8">{short}</text>')

    lines = []
    legend = []
    for idx, (name, values) in enumerate(series):
        color = palette[idx % len(palette)]
        points = " ".join(f"{x_pos(i):.1f},{y_pos(v):.1f}" for i, v in enumerate(values))
        lines.append(f'<polyline fill="none" stroke="{color}" stroke-width="2.5" points="{points}" />')
        for i, v in enumerate(values):
            lines.append(f'<circle cx="{x_pos(i):.1f}" cy="{y_pos(v):.1f}" r="3.2" fill="{color}" />')
        legend_y = margin + 8 + idx * 20
        legend.append(f'<rect x="{width - 290}" y="{legend_y - 10}" width="14" height="14" rx="2" fill="{color}" />')
        legend.append(f'<text x="{width - 268}" y="{legend_y + 1}" font-size="12" fill="#cbd5e1">{name}</text>')

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <rect width="100%" height="100%" fill="#0b1220"/>
  <text x="{margin}" y="42" font-size="24" fill="#f8fafc" font-family="Inter,system-ui,sans-serif">{title}</text>
  <text x="{margin}" y="64" font-size="12" fill="#94a3b8">Generated: {utc_now()}</text>
  {''.join(grid)}
  <line x1="{margin}" y1="{margin + chart_h}" x2="{margin + chart_w}" y2="{margin + chart_h}" stroke="#64748b" stroke-width="2"/>
  {''.join(lines)}
  {''.join(x_ticks)}
  {''.join(legend)}
</svg>
"""


def heatmap_svg(
    rows: list[StepRow],
    run_ids: list[str],
    steps: list[str],
    width: int = 1180,
    height: int = 500,
) -> str:
    margin = 120
    chart_w = width - 2 * margin
    chart_h = height - 2 * margin

    cell_w = chart_w / max(1, len(run_ids))
    cell_h = chart_h / max(1, len(steps))

    index = {(r.run_id, r.step): r.status for r in rows}
    blocks = []
    y_labels = []
    x_labels = []

    for yi, step in enumerate(steps):
        y = margin + yi * cell_h
        y_labels.append(
            f'<text x="{margin - 10}" y="{y + cell_h/2 + 4:.1f}" text-anchor="end" font-size="12" fill="#cbd5e1">{step}</text>'
        )
        for xi, run_id in enumerate(run_ids):
            x = margin + xi * cell_w
            status = index.get((run_id, step), "missing")
            color = "#10b981" if status == "pass" else "#ef4444" if status == "fail" else "#334155"
            blocks.append(f'<rect x="{x:.1f}" y="{y:.1f}" width="{cell_w - 4:.1f}" height="{cell_h - 4:.1f}" fill="{color}" rx="4" />')

    for xi, run_id in enumerate(run_ids):
        x = margin + xi * cell_w + cell_w / 2
        x_labels.append(
            f'<text x="{x:.1f}" y="{margin + chart_h + 20}" text-anchor="middle" font-size="10" fill="#94a3b8">{run_id.removeprefix("proof-")}</text>'
        )

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <rect width="100%" height="100%" fill="#0b1220"/>
  <text x="{margin}" y="42" font-size="24" fill="#f8fafc" font-family="Inter,system-ui,sans-serif">Run Stability Heatmap</text>
  <text x="{margin}" y="64" font-size="12" fill="#94a3b8">Green=pass, Red=fail, Gray=missing • Generated: {utc_now()}</text>
  {''.join(blocks)}
  {''.join(y_labels)}
  {''.join(x_labels)}
</svg>
"""


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--raw-dir", required=True, help="Path to raw proof status CSV files")
    parser.add_argument("--output", required=True, help="Output chart directory")
    args = parser.parse_args()

    raw_dir = Path(args.raw_dir)
    out_dir = Path(args.output)
    rows = read_history(raw_dir)
    if not rows:
        raise SystemExit(f"No status CSV rows found in {raw_dir}")

    run_ids = grouped_runs(rows)
    steps = sorted({r.step for r in rows})

    pass_rates = pass_rate_per_run(rows, run_ids)
    write_svg(
        out_dir / "verification-pass-rate-trend.svg",
        line_chart_svg(
            "Verification Gate Pass Rate by Run (%)",
            [("pass_rate_pct", [v for _, v in pass_rates])],
            [rid for rid, _ in pass_rates],
            y_max=100.0,
        ),
    )

    # Duration series by step
    duration_series: list[tuple[str, list[float]]] = []
    for step in steps:
        vals = []
        for run_id in run_ids:
            hit = next((r for r in rows if r.run_id == run_id and r.step == step), None)
            vals.append(hit.duration_sec if hit else 0.0)
        duration_series.append((step, vals))
    max_duration = max((v for _, vs in duration_series for v in vs), default=1.0)
    write_svg(
        out_dir / "step-duration-trend.svg",
        line_chart_svg("Step Duration Trend (seconds)", duration_series, run_ids, y_max=max_duration * 1.1),
    )

    write_svg(out_dir / "run-stability-heatmap.svg", heatmap_svg(rows, run_ids, steps))
    print(f"Generated 3 trend chart(s) in {out_dir}")


if __name__ == "__main__":
    main()
