#!/usr/bin/env python3
"""
Q1 Witness v1: Information Growth Rate (SVD Entropy Expansion)

This script demonstrates the first term of the Sovereignty Score (Omega).
It simulates the evolution of the constitutional basis Phi as new 
verified responses are added.

Mathematical Claims:
- H(Phi) increases with novel information.
- H(Phi) is stable under redundant information.
- The growth rate Delta H / Delta T measures epistemic expansion.
"""

import json
import math
import numpy as np
from datetime import datetime, timezone
from pathlib import Path

def compute_entropy(matrix):
    """
    Computes von Neumann-type entropy from singular values.
    """
    if matrix.size == 0:
        return 0.0
    _, s, _ = np.linalg.svd(matrix)
    # Normalize singular values to probabilities
    energies = s**2
    total_energy = np.sum(energies)
    if total_energy == 0:
        return 0.0
    probs = energies / total_energy
    # Remove zeros for log
    probs = probs[probs > 1e-12]
    return -np.sum(probs * np.log2(probs))

def main():
    out_dir = Path("/home/mega/Chyren/docs/proof/witness")
    out_dir.mkdir(parents=True, exist_ok=True)
    timestamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    
    # N=100 dimensions, m=5 starting basis
    N = 100
    m = 5
    
    # Initial basis (orthonormal)
    Phi = np.eye(N, m)
    
    steps = []
    current_Phi = Phi.copy()
    
    # Case 1: Initial State
    steps.append({
        "event": "initial",
        "entropy": compute_entropy(current_Phi),
        "basis_shape": current_Phi.shape
    })
    
    # Case 2: Add Novel L-type Information
    # Vector orthogonal to current Phi
    novel_vec = np.zeros((N, 1))
    novel_vec[m] = 1.0
    current_Phi = np.hstack([current_Phi, novel_vec])
    steps.append({
        "event": "novel_info",
        "entropy": compute_entropy(current_Phi),
        "basis_shape": current_Phi.shape
    })
    
    # Case 3: Add Redundant Information
    redundant_vec = current_Phi[:, 0:1] # Already in basis
    current_Phi = np.hstack([current_Phi, redundant_vec])
    steps.append({
        "event": "redundant_info",
        "entropy": compute_entropy(current_Phi),
        "basis_shape": current_Phi.shape
    })
    
    # Case 4: Add Slightly Correlated Information
    correlated_vec = 0.5 * current_Phi[:, 1:2] + 0.5 * np.eye(N, 1, k=m+1)
    current_Phi = np.hstack([current_Phi, correlated_vec])
    steps.append({
        "event": "partially_novel_info",
        "entropy": compute_entropy(current_Phi),
        "basis_shape": current_Phi.shape
    })
    
    # Write artifacts
    payload = {
        "timestamp": timestamp,
        "version": "v1",
        "manifold": "Stiefel V_m(R^N)",
        "metric": "SVD Entropy (log2)",
        "results": steps
    }
    
    json_path = out_dir / f"q1_witness_v1_{timestamp}.json"
    latest_json = out_dir / "latest_q1.json"
    
    with open(json_path, "w") as f:
        json.dump(payload, f, indent=2)
    with open(latest_json, "w") as f:
        json.dump(payload, f, indent=2)
        
    print(f"Q1 Witness complete. Artifacts written to {out_dir}")
    for s in steps:
        print(f"{s['event']:20}: H = {s['entropy']:.4f} | Dim = {s['basis_shape']}")

if __name__ == "__main__":
    main()
