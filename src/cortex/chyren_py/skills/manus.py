import os
import requests
import json
import time
from typing import Optional, Dict, Any

class ManusBrowserSkill:
    """
    ARI Skill for interacting with the Manus.ai General Purpose AI Agent.
    Enables Chyren to perform complex browser-based tasks.
    """
    
    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or os.getenv("MANUS_API_KEY")
        self.base_url = "https://api.manus.ai/v1"
        
        if not self.api_key:
            raise ValueError("MANUS_API_KEY is not set. Sovereignty requires valid credentials.")

    def run_task(self, goal: str, wait: bool = True) -> Dict[str, Any]:
        """
        Dispatches a browser-based goal to Manus.
        """
        headers = {
            "Authorization": f"Bearer {self.api_key}",
            "Content-Type": "application/json"
        }
        
        payload = {
            "goal": goal
        }
        
        try:
            response = requests.post(f"{self.base_url}/tasks", headers=headers, json=payload)
            response.raise_for_status()
            task_data = response.json()
            task_id = task_data.get("id")
            
            if not wait:
                return task_data
            
            # Poll for completion
            return self._wait_for_task(task_id)
            
        except Exception as e:
            return {"error": f"Manus task dispatch failed: {str(e)}", "status": "failed"}

    def _wait_for_task(self, task_id: str, timeout: int = 300) -> Dict[str, Any]:
        """
        Wait for a Manus task to reach a terminal state.
        """
        headers = {"Authorization": f"Bearer {self.api_key}"}
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            try:
                resp = requests.get(f"{self.base_url}/tasks/{task_id}", headers=headers)
                resp.raise_for_status()
                data = resp.json()
                
                status = data.get("status")
                if status in ["completed", "failed"]:
                    return data
                
                time.sleep(5)
            except Exception as e:
                return {"error": f"Error polling Manus task: {str(e)}", "status": "failed"}
                
        return {"error": "Manus task timed out", "status": "failed"}

def gather_social_creds_strategy(goal: str):
    """
    Helper function for Chyren to execute the social media credential gathering strategy.
    """
    skill = ManusBrowserSkill()
    
    # Strategy implementation
    full_goal = (
        f"Goal: {goal}. "
        "Find the profile links for each social media account. "
        "If possible, identify associated usernames. "
        "Output the results in a structured JSON format with 'platform', 'link', and 'username' keys."
    )
    
    return skill.run_task(full_goal)
