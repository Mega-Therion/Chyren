#!/usr/bin/env python3
"""
Run the first local Q5 witness computation.

This script does not claim to prove Q5. It implements the smallest reproducible
experiment described in the repo-local proof package:

- a commuting drift case
- a noncommuting drift case
- a control case with the bridge disabled

The "transport" here is a simple discretized loop-ordered product over a fixed
closed path parameterized by theta in [0, 2*pi]. The result is a witness
artifact, not a theorem.
"""

from __future__ import annotations

import csv
import json
import math
from cmath import phase
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path


Matrix = list[list[complex]]


def identity(n: int) -> Matrix:
    return [[1.0 + 0.0j if i == j else 0.0 + 0.0j for j in range(n)] for i in range(n)]


def matrix_add(a: Matrix, b: Matrix) -> Matrix:
    return [[a[i][j] + b[i][j] for j in range(len(a[0]))] for i in range(len(a))]


def scalar_mul(s: complex, a: Matrix) -> Matrix:
    return [[s * a[i][j] for j in range(len(a[0]))] for i in range(len(a))]


def matrix_mul(a: Matrix, b: Matrix) -> Matrix:
    rows = len(a)
    cols = len(b[0])
    inner = len(b)
    out = [[0.0 + 0.0j for _ in range(cols)] for _ in range(rows)]
    for i in range(rows):
        for k in range(inner):
            aik = a[i][k]
            for j in range(cols):
                out[i][j] += aik * b[k][j]
    return out


def matrix_sub(a: Matrix, b: Matrix) -> Matrix:
    return [[a[i][j] - b[i][j] for j in range(len(a[0]))] for i in range(len(a))]


def frobenius_norm(a: Matrix) -> float:
    return math.sqrt(sum(abs(entry) ** 2 for row in a for entry in row))


def trace(a: Matrix) -> complex:
    return sum(a[i][i] for i in range(len(a)))


def commutator(a: Matrix, b: Matrix) -> Matrix:
    return matrix_sub(matrix_mul(a, b), matrix_mul(b, a))


def serialize_matrix(a: Matrix) -> list[list[dict[str, float]]]:
    return [
        [{"re": float(entry.real), "im": float(entry.imag)} for entry in row]
        for row in a
    ]


def blend_operator(l1: Matrix, l2: Matrix, theta: float) -> Matrix:
    return matrix_add(scalar_mul(math.cos(theta), l1), scalar_mul(math.sin(theta), l2))


def transport(l1: Matrix, l2: Matrix, n_steps: int, bridge_enabled: bool) -> Matrix:
    d = len(l1)
    state = identity(d)
    dt = 2.0 * math.pi / n_steps
    if not bridge_enabled:
        return state

    for step in range(n_steps):
        theta = step * dt
        generator = blend_operator(l1, l2, theta)
        increment = matrix_add(identity(d), scalar_mul(dt, generator))
        state = matrix_mul(increment, state)
    return state


@dataclass
class CaseResult:
    case_id: str
    dimension: int
    bridge_enabled: bool
    n_steps: int
    path_id: str
    transport_rule_id: str
    l1: list[list[dict[str, float]]]
    l2: list[list[dict[str, float]]]
    commutator: list[list[dict[str, float]]]
    commutator_norm: float
    transport_output: list[list[dict[str, float]]]
    deviation_from_identity: float
    trace_re: float
    trace_im: float
    trace_phase: float


def run_case(case_id: str, l1: Matrix, l2: Matrix, *, bridge_enabled: bool, n_steps: int) -> CaseResult:
    output = transport(l1, l2, n_steps=n_steps, bridge_enabled=bridge_enabled)
    comm = commutator(l1, l2)
    tr = trace(output)
    return CaseResult(
        case_id=case_id,
        dimension=len(l1),
        bridge_enabled=bridge_enabled,
        n_steps=n_steps,
        path_id="theta_loop_[0,2pi]",
        transport_rule_id="left_product_euler_transport_v1",
        l1=serialize_matrix(l1),
        l2=serialize_matrix(l2),
        commutator=serialize_matrix(comm),
        commutator_norm=frobenius_norm(comm),
        transport_output=serialize_matrix(output),
        deviation_from_identity=frobenius_norm(matrix_sub(output, identity(len(l1)))),
        trace_re=float(tr.real),
        trace_im=float(tr.imag),
        trace_phase=float(phase(tr)) if abs(tr) > 0.0 else 0.0,
    )


def write_markdown(path: Path, timestamp: str, results: list[CaseResult]) -> None:
    lines = [
        "# Q5 Witness Run",
        "",
        f"- timestamp: `{timestamp}`",
        "- path_id: `theta_loop_[0,2pi]`",
        "- transport_rule_id: `left_product_euler_transport_v1`",
        "",
        "| case_id | bridge_enabled | commutator_norm | deviation_from_identity | trace_phase |",
        "| --- | --- | ---: | ---: | ---: |",
    ]
    for result in results:
        lines.append(
            f"| `{result.case_id}` | `{result.bridge_enabled}` | "
            f"{result.commutator_norm:.6f} | {result.deviation_from_identity:.6f} | "
            f"{result.trace_phase:.6f} |"
        )
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def write_csv(path: Path, results: list[CaseResult]) -> None:
    with path.open("w", newline="", encoding="utf-8") as fh:
        writer = csv.DictWriter(
            fh,
            fieldnames=[
                "case_id",
                "dimension",
                "bridge_enabled",
                "n_steps",
                "path_id",
                "transport_rule_id",
                "commutator_norm",
                "deviation_from_identity",
                "trace_re",
                "trace_im",
                "trace_phase",
            ],
        )
        writer.writeheader()
        for result in results:
            row = asdict(result)
            writer.writerow({key: row[key] for key in writer.fieldnames})


def main() -> None:
    out_dir = Path("/home/mega/Chyren/docs/proof/witness")
    out_dir.mkdir(parents=True, exist_ok=True)

    timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    n_steps = 512

    commuting_l1 = [[1.0 + 0.0j, 0.0 + 0.0j], [0.0 + 0.0j, -1.0 + 0.0j]]
    commuting_l2 = [[0.5 + 0.0j, 0.0 + 0.0j], [0.0 + 0.0j, 2.0 + 0.0j]]
    noncommuting_l1 = [[0.0 + 0.0j, 1.0 + 0.0j], [1.0 + 0.0j, 0.0 + 0.0j]]
    noncommuting_l2 = [[0.0 + 0.0j, -1.0j], [1.0j, 0.0 + 0.0j]]

    results = [
        run_case("commuting", commuting_l1, commuting_l2, bridge_enabled=True, n_steps=n_steps),
        run_case(
            "noncommuting",
            noncommuting_l1,
            noncommuting_l2,
            bridge_enabled=True,
            n_steps=n_steps,
        ),
        run_case("control", noncommuting_l1, noncommuting_l2, bridge_enabled=False, n_steps=n_steps),
    ]

    json_path = out_dir / f"q5_witness_{timestamp}.json"
    csv_path = out_dir / f"q5_witness_{timestamp}.csv"
    md_path = out_dir / f"q5_witness_{timestamp}.md"
    latest_json = out_dir / "latest.json"
    latest_csv = out_dir / "latest.csv"
    latest_md = out_dir / "latest.md"

    payload = {
        "timestamp": timestamp,
        "path_id": "theta_loop_[0,2pi]",
        "transport_rule_id": "left_product_euler_transport_v1",
        "n_steps": n_steps,
        "results": [asdict(result) for result in results],
    }

    json_path.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    latest_json.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    write_csv(csv_path, results)
    write_csv(latest_csv, results)
    write_markdown(md_path, timestamp, results)
    write_markdown(latest_md, timestamp, results)

    print(json.dumps({"json": str(json_path), "csv": str(csv_path), "md": str(md_path)}, indent=2))


if __name__ == "__main__":
    main()
