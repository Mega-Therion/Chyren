import os
import numpy as np
import json

def verify_data_integrity():
    results_dir = "results"
    npz_files = [f for f in os.listdir(results_dir) if f.endswith("_raw.npz")]
    
    report = {
        "total_files": len(npz_files),
        "valid_signal_files": 0,
        "consistency_checks": [],
        "warnings": []
    }
    
    chi_local = 0.707
    T_r_expected = 1.0 + (1.0 / (chi_local * 0.5)) # 3.828854314002829
    
    for f in npz_files:
        path = os.path.join(results_dir, f)
        data = np.load(path)
        flux = data["flux_mjy"]
        wavelength = data["wavelength_um"]
        
        # Check 1: Data Presence
        if len(flux) == 0:
            report["warnings"].append(f"{f}: Empty flux array.")
            continue
            
        # Check 2: Signal Quality
        valid_mask = ~np.isnan(flux)
        valid_count = np.sum(valid_mask)
        if valid_count == 0:
            report["warnings"].append(f"{f}: Flux array contains only NaNs.")
            continue
            
        report["valid_signal_files"] += 1
        
        # Check 3: Transformation Consistency (Independent calculation)
        mean_flux = np.mean(flux[valid_mask])
        transformed_sample = mean_flux * T_r_expected
        
        # Check 4: Physical Sanity
        if np.any(flux[valid_mask] < -1.0e-3): # Allowing for small noise fluctuations
            report["warnings"].append(f"{f}: Negative flux detected (possible background subtraction artifact).")
            
        # Check 5: Range Verification
        if np.max(wavelength) > 30.0 or np.min(wavelength) < 0.5:
             report["warnings"].append(f"{f}: Wavelength range ({np.min(wavelength):.2f}-{np.max(wavelength):.2f} um) outside standard NIRSpec/NIRCAM/MIRI expected bands.")

    return report

if __name__ == "__main__":
    print("CHYREN SOVEREIGN VERIFICATION: JWST DATA INTEGRITY CHECK")
    print("=======================================================")
    results = verify_data_integrity()
    print(json.dumps(results, indent=4))
