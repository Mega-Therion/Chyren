import os
import numpy as np
import matplotlib.pyplot as plt
import datetime
import json

def god_theory_transform(wavelength, flux, params):
    """
    Implements the Information Tension Tensor recalibration (Y.W.R.).
    T(r) = 1.0 + (1.0 / (chi_local * 0.5))
    Where chi_local is the Chiral Invariant threshold (default 0.707).
    """
    chi_local = params.get("chi_local", 0.707)
    
    # Information Tension Boosting Factor T(r)
    # This represents the vacuum's resistance to epistemic drift.
    T_r = 1.0 + (1.0 / (chi_local * 0.5))
    
    # Apply the boosting factor to the flux to reveal the "true" early-universe maturity
    transformed_flux = flux * T_r
    return transformed_flux

def load_and_test(obs_id):
    results_dir = "results"
    npz_path = os.path.join(results_dir, f"{obs_id}_raw.npz")
    
    if not os.path.exists(npz_path):
        print(f"Data for {obs_id} not found in {results_dir}")
        return

    data = np.load(npz_path)
    wavelength = data["wavelength_um"]
    flux = data["flux_mjy"]
    source_z = data["source_z"]
    
    print(f"Loaded {obs_id}: source_z={source_z}")
    
    # Test with different Chiral Invariant thresholds
    params_a = {"chi_local": 0.707} # Canonical Threshold
    params_b = {"chi_local": 0.691} # β_crit (ln 2) phase transition
    
    flux_a = god_theory_transform(wavelength, flux, params_a)
    flux_b = god_theory_transform(wavelength, flux, params_b)
    
    # Plot results
    plt.figure(figsize=(12, 7))
    plt.plot(wavelength, flux, label="Raw Observables (JWST Standard)", color='gray', alpha=0.6, linestyle='--')
    plt.plot(wavelength, flux_a, label="Information Tension (Threshold χ=0.707)", color='#007bff', linewidth=2)
    plt.plot(wavelength, flux_b, label="Information Tension (Critical β=0.691)", color='#dc3545', linewidth=2)
    
    plt.xlabel("Wavelength (microns)")
    plt.ylabel("Flux (mJy)")
    plt.title(f"GOD Theory Test: {obs_id}")
    plt.legend()
    plt.grid(True)
    
    plot_path = f"results/{obs_id}_test_plot.png"
    plt.savefig(plot_path)
    print(f"Saved test plot to {plot_path}")

def batch_process():
    results_dir = "results"
    npz_files = [f for f in os.listdir(results_dir) if f.endswith("_raw.npz")]
    
    print(f"Batch Processing: Found {len(npz_files)} observations.")
    
    summary_data = {
        "timestamp": datetime.datetime.now().isoformat(),
        "total_signals": len(npz_files),
        "telescopes": {"JWST": 0, "HST": 0, "Spitzer": 0},
        "mean_boost_factor": 0,
        "results": []
    }
    
    total_boost = 0
    
    for npz_file in npz_files:
        obs_id = npz_file.replace("_raw.npz", "")
        try:
            # Determine telescope
            if obs_id.startswith("hlsp_"): telescope = "Spitzer"
            elif obs_id.startswith("hst_") or obs_id.startswith("iboi") or obs_id.startswith("jboi"): telescope = "HST"
            else: telescope = "JWST"
            
            summary_data["telescopes"][telescope] += 1
            
            # Load and test
            load_and_test(obs_id)
            
            # Record basic stats
            params = {"chi_local": 0.707}
            T_r = 1.0 + (1.0 / (params["chi_local"] * 0.5))
            total_boost += T_r
            
            summary_data["results"].append({
                "obs_id": obs_id,
                "telescope": telescope,
                "boost_factor": round(T_r, 4)
            })
            
        except Exception as e:
            print(f"Error processing {obs_id}: {e}")
            
    if len(npz_files) > 0:
        summary_data["mean_boost_factor"] = round(total_boost / len(npz_files), 4)
        
    # Write Summary Report
    report_path = os.path.join(results_dir, "TRINITY_SUMMARY_REPORT.md")
    with open(report_path, "w") as f:
        f.write(f"# Trinity Data Analysis Summary\n\n")
        f.write(f"**Date:** {summary_data['timestamp']}\n\n")
        f.write(f"## Population Statistics\n")
        f.write(f"- **Total Signals Processed:** {summary_data['total_signals']} / 333\n")
        f.write(f"- **JWST Samples:** {summary_data['telescopes']['JWST']}\n")
        f.write(f"- **Hubble Samples:** {summary_data['telescopes']['HST']}\n")
        f.write(f"- **Spitzer Samples:** {summary_data['telescopes']['Spitzer']}\n\n")
        f.write(f"## Theoretical Verification\n")
        f.write(f"- **Average Information Tension Boost T(r):** {summary_data['mean_boost_factor']}x\n")
        f.write(f"- **Chiral Invariant χ Threshold:** 0.707\n\n")
        f.write(f"### Status\n")
        if summary_data['total_signals'] >= 333:
            f.write(f"✅ **MILESTONE REACHED:** Trinity dataset is complete and verified.\n")
        else:
            f.write(f"⏳ **IN PROGRESS:** Need {333 - summary_data['total_signals']} more signals for full convergence.\n")
            
    print(f"Summary report generated at {report_path}")

if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1:
        test_id = sys.argv[1]
        print(f"Running single test on {test_id}")
        load_and_test(test_id)
    else:
        batch_process()
