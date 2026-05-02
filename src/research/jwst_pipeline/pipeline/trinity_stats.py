import os
import numpy as np
import json

def get_stats():
    results_dir = "results"
    npz_files = [f for f in os.listdir(results_dir) if f.endswith("_raw.npz")]
    
    boosts = []
    instruments = {}
    
    for f in npz_files:
        path = os.path.join(results_dir, f)
        try:
            data = np.load(path)
            flux = data["flux_mjy"]
            if len(flux) > 0 and not np.all(np.isnan(flux)):
                # In the theory, mean flux is a proxy for the energy density
                m = np.nanmean(flux)
                if m > 0:
                    boosts.append(m)
                    
                    # Robust instrument detection
                    fname = f.lower()
                    if "jw" in fname or "v0" in fname:
                        instruments["JWST"] = instruments.get("JWST", 0) + 1
                    elif "hst" in fname or "ibo" in fname or fname.startswith("j") or fname.startswith("i"):
                        instruments["Hubble"] = instruments.get("Hubble", 0) + 1
                    elif "hlsp" in fname or "spitzer" in fname:
                        instruments["Spitzer"] = instruments.get("Spitzer", 0) + 1
                    else:
                        instruments["Other/Unknown"] = instruments.get("Other/Unknown", 0) + 1
        except:
            continue
            
    stats = {
        "total_verified": len(boosts),
        "mean_boost_proxy": np.mean(boosts) if boosts else 0,
        "std_dev": np.std(boosts) if boosts else 0,
        "instruments": instruments
    }
    return stats

if __name__ == "__main__":
    print(json.dumps(get_stats(), indent=4))
