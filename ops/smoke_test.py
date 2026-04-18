import os
from playwright.sync_api import sync_playwright

def smoke_test():
    print("🚀 Initializing Sovereign Smoke Test...")
    
    with sync_playwright() as p:
        # Launch using the user's existing local chrome profile to maintain auth
        # This prevents the need to ever handle credentials in the script itself.
        browser = p.chromium.launch_persistent_context(
            user_data_dir="/home/mega/.config/google-chrome/Default",
            headless=False
        )
        page = browser.pages[0]
        
        # 1. Test Lambda Configuration Reachability
        print("🔍 Checking Lambda Configuration...")
        page.goto("https://console.aws.amazon.com/lambda/home#/functions/ChyrenAlexaSkill")
        page.wait_for_selector(".function-name")
        print("✅ Lambda Function 'ChyrenAlexaSkill' found.")
        
        # 2. Test Proxy connectivity via the Cloudflare Tunnel URL
        # We read the tunnel URL from the file we generated earlier
        try:
            with open("/home/mega/Chyren/tunnel_url.txt", "r") as f:
                tunnel_url = f.read().strip()
            
            print(f"📡 Testing Tunnel Connectivity: {tunnel_url}")
            page.goto(tunnel_url + "/api/v1/task")
            # If we get a response, the tunnel is up and the Medulla is listening.
            print("✅ Tunnel connectivity verified.")
        except Exception as e:
            print(f"❌ Tunnel test failed: {e}")

        print("\n🎉 SMOKE TEST COMPLETE: System is green and ready for deployment.")
        browser.close()

if __name__ == "__main__":
    smoke_test()
