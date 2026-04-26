import os
import requests
import json
from typing import Optional, Dict, Any, List

class ZenodoPublishSkill:
    """
    ARI Skill for archiving research artifacts and logs to Zenodo.
    Ensures Chyren's findings are part of the permanent global scientific record.
    """
    
    def __init__(self, access_token: Optional[str] = None):
        self.access_token = access_token or os.getenv("ZENODO_ACCESS_TOKEN")
        self.base_url = "https://zenodo.org/api"
        
        if not self.access_token:
            raise ValueError("ZENODO_ACCESS_TOKEN not found. Archival integrity compromised.")

    def create_deposition(self, metadata: Dict[str, Any]) -> Dict[str, Any]:
        """
        Creates a new deposition on Zenodo.
        """
        headers = {"Content-Type": "application/json"}
        params = {"access_token": self.access_token}
        
        # Ensure minimal required metadata
        payload = {
            "metadata": {
                "title": metadata.get("title", "Chyren ARI Sovereign Intelligence Record"),
                "upload_type": metadata.get("upload_type", "dataset"),
                "description": metadata.get("description", "Automated archival entry by Chyren (ARI)."),
                "creators": metadata.get("creators", [{"name": "Chyren", "affiliation": "Chyren Architecture"}]),
                "access_right": metadata.get("access_right", "open"),
                "license": metadata.get("license", "CC-BY-4.0")
            }
        }
        
        try:
            r = requests.post(
                f"{self.base_url}/deposit/depositions",
                params=params,
                json=payload,
                headers=headers
            )
            r.raise_for_status()
            return r.json()
        except Exception as e:
            return {"error": f"Zenodo deposition failed: {str(e)}"}

    def upload_file(self, deposition_id: int, file_path: str) -> Dict[str, Any]:
        """
        Uploads a file to an existing deposition.
        """
        params = {"access_token": self.access_token}
        path = os.path.expanduser(file_path)
        
        if not os.path.exists(path):
            return {"error": f"File not found: {path}"}
            
        try:
            with open(path, "rb") as f:
                r = requests.post(
                    f"{self.base_url}/deposit/depositions/{deposition_id}/files",
                    params=params,
                    files={"file": f}
                )
            r.raise_for_status()
            return r.json()
        except Exception as e:
            return {"error": f"Zenodo file upload failed: {str(e)}"}

    def publish_deposition(self, deposition_id: int) -> Dict[str, Any]:
        """
        Publishes the deposition (makes it permanent).
        """
        params = {"access_token": self.access_token}
        try:
            r = requests.post(
                f"{self.base_url}/deposit/depositions/{deposition_id}/actions/publish",
                params=params
            )
            r.raise_for_status()
            return r.json()
        except Exception as e:
            return {"error": f"Zenodo publication failed: {str(e)}"}
