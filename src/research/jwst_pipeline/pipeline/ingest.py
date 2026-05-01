import os
import json
import logging
import datetime
import numpy as np
import psycopg2
from astropy.io import fits
from dotenv import load_dotenv

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

load_dotenv()

def ingest_data():
    raw_dir = os.getenv("RAW_DIR", "data/raw")
    processed_dir = "data/processed"
    results_dir = "results"
    
    logger.info(f"Raw directory: {raw_dir}")
    logger.info(f"Contents: {os.listdir(raw_dir)}")
    
    os.makedirs(processed_dir, exist_ok=True)
    os.makedirs(results_dir, exist_ok=True)

    # Database connection
    conn_url = os.getenv("CHYREN_JWST_DB_URL")
    conn = None
    if conn_url:
        try:
            conn = psycopg2.connect(conn_url)
            cursor = conn.cursor()
            logger.info("Connected to PostgreSQL")
        except Exception as e:
            logger.error(f"Failed to connect to DB: {e}")

    metadata_log = []

    # Iterate through downloaded programs
    for prog_id in os.listdir(raw_dir):
        prog_path = os.path.join(raw_dir, prog_id)
        if not os.path.isdir(prog_path):
            logger.warning(f"Skipping {prog_id}: Not a directory")
            continue
            
        logger.info(f"Processing Program ID: {prog_id}")
        
        for root, dirs, files in os.walk(prog_path):
            logger.info(f"Walking {root} (Files: {len(files)})")
            for file in files:
                if file.endswith(".fits"):
                    fits_path = os.path.join(root, file)
                    logger.info(f"Parsing {file}...")
                    
                    try:
                        with fits.open(fits_path) as hdul:
                            header = hdul[0].header
                            
                            # Extract metadata
                            raw_obs_id = header.get("OBS_ID", "Unknown")
                            obs_id = f"{raw_obs_id}_{file}"
                            target = header.get("TARGNAME", "Unknown")
                            ra = header.get("TARG_RA", header.get("RA_TARG", 0.0))
                            dec = header.get("TARG_DEC", header.get("DEC_TARG", 0.0))
                            source_z = header.get("SRC_RED", header.get("SRCRED", 0.0))
                            instrument = header.get("INSTRUME", "JWST")
                            filt = header.get("FILTER", "N/A")
                            exptime = header.get("EXPTIME", 0.0)

                            # Extract spectral data (assuming CAL or X1D format)
                            # This is a simplified extraction of wavelength and flux
                            # Extract spectral data (Prioritize X1D/binary table format)
                            wavelength = []
                            flux = []
                            flux_err = []
                            
                            # Scan HDUs for binary tables with wavelength/flux
                            for hdu in hdul:
                                if hasattr(hdu, 'columns'):
                                    cols = hdu.columns.names
                                    logger.debug(f"Found columns: {cols}")
                                    if "WAVELENGTH" in cols and "FLUX" in cols:
                                        wavelength = hdu.data["WAVELENGTH"]
                                        flux = hdu.data["FLUX"]
                                        if "ERROR" in cols:
                                            flux_err = hdu.data["ERROR"]
                                        elif "ERR" in cols:
                                            flux_err = hdu.data["ERR"]
                                        break
                                    # Fallback for different column naming conventions
                                    elif "wave" in [c.lower() for c in cols] and "flux" in [c.lower() for c in cols]:
                                        wave_col = [c for c in cols if c.lower() == "wave"][0]
                                        flux_col = [c for c in cols if c.lower() == "flux"][0]
                                        wavelength = hdu.data[wave_col]
                                        flux = hdu.data[flux_col]
                                        break
                                elif (hdu.name == "SCI" or (hdu.is_image and hdu.data is not None)) and len(hdu.data.shape) == 2:
                                    # BEST EFFORT: Extract 1D profile from 2D CAL image
                                    logger.info(f"Found 2D array in HDU {hdu.name}. Collapsing to 1D profile.")
                                    
                                    # Factor for Spitzer 'Factory Settings' (Gain/BUNIT)
                                    bunit = header.get("BUNIT", "unknown")
                                    gain = header.get("GAIN", 1.0)
                                    if "Spitzer" in str(prog_id):
                                        logger.info(f"Applying Spitzer Recalibration (BUNIT: {bunit}, GAIN: {gain})")
                                        # Convert MJy/sr or similar to standardized counts if needed
                                        hdu.data = hdu.data * gain
                                        
                                    flux = np.nanmean(hdu.data, axis=0) # Mean along spatial axis
                                    # Generate dummy wavelength if missing (microns)
                                    wavelength = np.linspace(0.5, 25.0, len(flux)) 
                                    break

                            # Save to compressed NumPy array
                            npz_filename = f"{obs_id}_raw.npz"
                            npz_path = os.path.join(results_dir, npz_filename)
                            
                            np.savez_compressed(npz_path, 
                                               wavelength_um=wavelength,
                                               flux_mjy=flux,
                                               flux_err=flux_err,
                                               source_z=source_z)
                            
                            # Log metadata
                            entry = {
                                "obs_id": obs_id,
                                "program_id": prog_id,
                                "target": target,
                                "ra": ra,
                                "dec": dec,
                                "source_z": source_z,
                                "instrument": instrument,
                                "filter": filt,
                                "npz_path": npz_path,
                                "timestamp": datetime.datetime.now().isoformat()
                            }
                            metadata_log.append(entry)

                            # Save to Database
                            if conn:
                                cursor.execute("""
                                    INSERT INTO jwst_observations 
                                    (obs_id, program_id, target_name, ra, dec, source_z, instrument, filter, exposure_time, data_path)
                                    VALUES (%s, %s, %s, %s, %s, %s, %s, %s, %s, %s)
                                    ON CONFLICT (obs_id) DO UPDATE SET processed_at = CURRENT_TIMESTAMP
                                """, (obs_id, str(prog_id), target, ra, dec, source_z, instrument, filt, exptime, npz_path))
                                conn.commit()

                            # AUTO-PURGE: Delete raw FITS after successful NPZ creation
                            if os.path.exists(npz_path):
                                os.remove(fits_path)
                                logger.info(f"Purged raw file: {file}")

                    except Exception as e:
                        logger.error(f"Error processing {file}: {e}")

    # Save metadata log
    log_path = os.path.join(processed_dir, f"metadata_{datetime.datetime.now().strftime('%Y%m%d_%H%M%S')}.json")
    with open(log_path, 'w') as f:
        json.dump(metadata_log, f, indent=4)
    
    logger.info(f"Ingestion complete. Processed {len(metadata_log)} observations.")
    if conn:
        conn.close()

if __name__ == "__main__":
    ingest_data()
