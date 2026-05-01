import numpy as np
import matplotlib.pyplot as plt
import os

def analyze_phase_transition():
    results_dir = "results"
    chi_values = []
    
    for f in os.listdir(results_dir):
        if f.endswith("_raw.npz"):
            try:
                data = np.load(os.path.join(results_dir, f))
                flux = data['flux_mjy']
                flux = flux[np.isfinite(flux)] # Filter NaNs
                if len(flux) > 0:
                    flux_norm = flux / (np.max(flux) + 1e-10)
                    chi_local = np.mean(flux_norm)
                    if np.isfinite(chi_local):
                        chi_values.append(chi_local)
            except:
                continue
                
    if not chi_values:
        print("No chi values found.")
        return
        
    chi_values = np.array(chi_values)
    print(f"Mean Chi: {np.mean(chi_values)}")
    print(f"Median Chi: {np.median(chi_values)}")
    print(f"Std Chi: {np.std(chi_values)}")
    
    # Check for Phase Transition near chi = 0.707 or beta = 0.691
    # We look for a clustering or a sharp drop-off.
    
    plt.figure(figsize=(10, 6))
    plt.hist(chi_values, bins=50, alpha=0.7, color='purple', label='Trinity Signals')
    plt.axvline(0.707, color='red', linestyle='--', label='Theoretical Threshold (0.707)')
    plt.title("Distribution of Chiral Invariants across Trinity Dataset")
    plt.xlabel("Chi (χ)")
    plt.ylabel("Count")
    plt.legend()
    plt.savefig("results/CHI_DISTRIBUTION.png")
    print("Saved distribution plot to results/CHI_DISTRIBUTION.png")

if __name__ == "__main__":
    analyze_phase_transition()
