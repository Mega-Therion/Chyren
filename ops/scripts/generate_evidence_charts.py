#!/usr/bin/env python3
"""Generate simple SVG charts from evidence metrics CSV.

Dependency-free by design (standard library only).
"""

from __future__ import annotations

import argparse
import csv
import os
from collections import defaultdict
from datetime import datetime, timezone


def read_metrics(path: str) -> list[dict[str, str]]:
    with open(path, newline="", encoding="utf-8") as f:
        return list(csv.DictReader(f))


def to_float(value: str) -> float:
    try:
        return float(value)
    except ValueError:
        return 0.0


def write_svg(path: str, content: str) -> None:
    with open(path, "w", encoding="utf-8") as f:
        f.write(content)


def bar_chart_svg(title: str, data: list[tuple[str, float]], width: int = 980, height: int = 520) -> str:
    margin = 80
    chart_w = width - (2 * margin)
    chart_h = height - (2 * margin)
    max_v = max((v for _, v in data), default=1.0) or 1.0
    bar_gap = 24
    bar_w = max(24, int((chart_w - bar_gap * (len(data) + 1)) / max(1, len(data))))

    bars = []
    labels = []
    for i, (name, value) in enumerate(data):
        x = margin + bar_gap + i * (bar_w + bar_gap)
        h = int((value / max_v) * (chart_h - 10))
        y = margin + chart_h - h
        bars.append(f'<rect x="{x}" y="{y}" width="{bar_w}" height="{h}" fill="#4f46e5" rx="6" />')
        labels.append(
            f'<text x="{x + bar_w / 2:.1f}" y="{margin + chart_h + 22}" text-anchor="middle" font-size="12" fill="#cbd5e1">{name}</text>'
        )
        labels.append(
            f'<text x="{x + bar_w / 2:.1f}" y="{y - 8}" text-anchor="middle" font-size="12" fill="#e2e8f0">{value:g}</text>'
        )

    y_grid = []
    for tick in range(6):
        t_val = (max_v / 5) * tick
        y = margin + chart_h - int((t_val / max_v) * (chart_h - 10))
        y_grid.append(f'<line x1="{margin}" y1="{y}" x2="{margin + chart_w}" y2="{y}" stroke="#334155" stroke-width="1" />')
        y_grid.append(f'<text x="{margin - 10}" y="{y + 4}" text-anchor="end" font-size="11" fill="#94a3b8">{t_val:g}</text>')

    generated = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M:%SZ")
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">
  <rect width="100%" height="100%" fill="#0b1220"/>
  <text x="{margin}" y="42" font-family="Inter,system-ui,sans-serif" font-size="24" fill="#f8fafc">{title}</text>
  <text x="{margin}" y="64" font-family="Inter,system-ui,sans-serif" font-size="12" fill="#94a3b8">Generated: {generated}</text>
  {''.join(y_grid)}
  <line x1="{margin}" y1="{margin + chart_h}" x2="{margin + chart_w}" y2="{margin + chart_h}" stroke="#64748b" stroke-width="2" />
  {''.join(bars)}
  {''.join(labels)}
</svg>
"""


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True, help="Path to metrics.csv")
    parser.add_argument("--output", required=True, help="Output directory for SVG charts")
    args = parser.parse_args()

    rows = read_metrics(args.input)
    os.makedirs(args.output, exist_ok=True)

    per_category: dict[str, list[tuple[str, float]]] = defaultdict(list)
    for row in rows:
        category = row.get("category", "uncategorized")
        per_category[category].append((row.get("metric_name", "metric"), to_float(row.get("value", "0"))))

    all_data = [(row.get("metric_name", "metric"), to_float(row.get("value", "0"))) for row in rows]
    write_svg(os.path.join(args.output, "all-metrics.svg"), bar_chart_svg("Chyren Proof Pack Metrics", all_data))

    for category, data in per_category.items():
        safe_name = category.lower().replace(" ", "-")
        write_svg(
            os.path.join(args.output, f"metrics-{safe_name}.svg"),
            bar_chart_svg(f"Chyren Metrics by Category: {category}", data),
        )

    print(f"Generated {1 + len(per_category)} chart(s) in {args.output}")


if __name__ == "__main__":
    main()
