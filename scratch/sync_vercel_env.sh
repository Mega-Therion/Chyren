#!/bin/bash
# Sync keys from ~/.omega/one-true.env to Vercel chyren-web

ENV_FILE="/home/mega/.omega/one-true.env"
PROJECT="chyren-web"

if [ ! -f "$ENV_FILE" ]; then
    echo "Error: $ENV_FILE not found."
    exit 1
fi

echo "Starting Vercel environment sync for $PROJECT..."

# Read only the lines that are key=value pairs and not comments
grep -v "^#" "$ENV_FILE" | grep "=" | while read -r line; do
    KEY=$(echo "$line" | cut -d'=' -f1)
    VALUE=$(echo "$line" | cut -d'=' -f2- | tr -d '"' | tr -d "'")
    
    if [ -n "$KEY" ] && [ -n "$VALUE" ]; then
        echo "Syncing $KEY..."
        # We use printf to handle special characters and pass to vercel env add
        # We specify production, preview, and development environments
        printf "%s" "$VALUE" | vercel env add "$KEY" production preview development --force 2>/dev/null
        if [ $? -eq 0 ]; then
            echo "Successfully synced $KEY."
        else
            # If add fails (e.g. already exists and --force didn't overwrite correctly), try remove and add
            vercel env rm "$KEY" production preview development -y 2>/dev/null
            printf "%s" "$VALUE" | vercel env add "$KEY" production preview development 2>/dev/null
            echo "Re-synced $KEY."
        fi
    fi
done

echo "Vercel sync complete."
