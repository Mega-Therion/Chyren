import http.server
import json
import os
from datetime import datetime

PORT = 8080
LOG_FILE = "manus_webhooks.log"

class ManusWebhookHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        post_data = self.rfile.read(content_length)
        
        try:
            data = json.loads(post_data)
            timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
            
            # Log the receipt
            with open(LOG_FILE, "a") as f:
                f.write(f"\n--- Webhook Received: {timestamp} ---\n")
                f.write(json.dumps(data, indent=2))
                f.write("\n" + "="*40 + "\n")
            
            # Print to console for immediate visibility
            print(f"\n[ARI] Manus Webhook Received at {timestamp}")
            print(f"Task ID: {data.get('id')}")
            print(f"Status: {data.get('status')}")
            if 'output' in data:
                print(f"Output Preview: {str(data['output'])[:200]}...")
            
            # Respond to Manus
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(json.dumps({"status": "received"}).encode())
            
        except Exception as e:
            print(f"Error processing webhook: {e}")
            self.send_response(500)
            self.end_headers()

def run_server():
    server_address = ('', PORT)
    httpd = http.server.HTTPServer(server_address, ManusWebhookHandler)
    print(f"─── CHYREN WEBHOOK GATEWAY ───")
    print(f"Listening on port {PORT}...")
    print(f"Awaiting Manus.ai callback transmissions.")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nShutting down gateway.")
        httpd.server_close()

if __name__ == "__main__":
    run_server()
