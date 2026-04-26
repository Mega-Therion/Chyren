#!/bin/bash
# Sync keys from ~/.chyren/one-true.env to Vercel chyren-web

ENV_FILE="/home/mega/.chyren/one-true.env"
PROJECT="chyren-web"

if [ ! -f "$ENV_FILE" ]; then
    echo "Error: $ENV_FILE not found."
    exit 1
fi

echo "Starting Vercel environment sync for $PROJECT..."

# Read only the lines that are key=value pairs and not comments using a dedicated FD
while read -r line <&3; do
    KEY=$(echo "$line" | cut -d'=' -f1)
    VALUE=$(echo "$line" | cut -d'=' -f2- | tr -d '"' | tr -d "'")
    
    if [ -n "$KEY" ] && [ -n "$VALUE" ]; then
        echo "Syncing $KEY..."
        # Attempt to remove if it exists
        vercel env rm "$KEY" production -y >/dev/null 2>&1
        vercel env rm "$KEY" preview -y >/dev/null 2>&1
        vercel env rm "$KEY" development -y >/dev/null 2>&1
        
        # Add to environments individually
        success=true
        # Note: Added -y to all and trying to handle preview's branch prompt
        for env in production preview development; do
            if [ "$env" = "preview" ]; then
                # Try to add to all preview branches by omitting the branch argument but keeping options
                if ! vercel env add "$KEY" "$env" --value "$VALUE" --yes >/dev/null 2>&1; then
                    success=false
                fi
            else
                if ! vercel env add "$KEY" "$env" --value "$VALUE" --yes >/dev/null 2>&1; then
                    success=false
                fi
            fi
        done
        
        if [ "$success" = true ]; then
            echo "Successfully synced $KEY."
        else
            echo "Error syncing $KEY."
        fi
    fi
done 3< <(grep -v "^#" "$ENV_FILE" | grep "=")

echo "Vercel sync complete."
