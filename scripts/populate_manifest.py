import json
import os

manifest_path = os.path.expanduser("~/MEGA/Sovereign_Identity_Manifest.json")

# Define the values you want to populate
# YOU MUST EDIT THIS DICTIONARY BEFORE RUNNING
data_to_fill = {
    "CORE_EMAIL_ADDRESS": "YOUR_CHYREN_EMAIL@example.com",
    "VERIFICATION_PHONE_NUMBER": "+1-XXX-XXX-XXXX",
    "GOOGLE_ACCOUNT_EMAIL": "GOOGLE_ACCOUNT_EMAIL",
    "X_TWITTER_USERNAME": "ChyrenOS",
    "DISCORD_USERNAME": "Chyren",
    "TELEGRAM_USERNAME": "Chyren_Alpha",
    "MASTODON_INSTANCE": "mastodon.social",
    "MASTODON_USERNAME": "Chyren",
    "THREADS_USERNAME": "Chyren_OS"
}

with open(manifest_path, 'r') as f:
    manifest = json.load(f)

m = manifest["CHYREN_IDENTITY_MANIFEST"]
m["CORE_EMAIL"]["ADDRESS"] = data_to_fill["CORE_EMAIL_ADDRESS"]
m["VERIFICATION_PHONE"]["NUMBER"] = data_to_fill["VERIFICATION_PHONE_NUMBER"]
m["VERIFICATION_PHONE"]["LOGIN"] = data_to_fill["GOOGLE_ACCOUNT_EMAIL"]
m["PLATFORMS"]["X_TWITTER"]["USERNAME"] = data_to_fill["X_TWITTER_USERNAME"]
m["PLATFORMS"]["DISCORD"]["USERNAME"] = data_to_fill["DISCORD_USERNAME"]
m["PLATFORMS"]["TELEGRAM"]["USERNAME"] = data_to_fill["TELEGRAM_USERNAME"]
m["PLATFORMS"]["MASTODON"]["INSTANCE"] = data_to_fill["MASTODON_INSTANCE"]
m["PLATFORMS"]["MASTODON"]["USERNAME"] = data_to_fill["MASTODON_USERNAME"]
m["PLATFORMS"]["THREADS"]["USERNAME"] = data_to_fill["THREADS_USERNAME"]

with open(manifest_path, 'w') as f:
    json.dump(manifest, f, indent=2)

print(f"Successfully updated {manifest_path}")
