#!/usr/bin/env bash
# Final Chyren Migration & Restructuring Script
# Migrates system-level Omega files into the local Chyren repo and rebrands them.

set -euo pipefail

CHYREN_DIR="/home/mega/Chyren"
CONFIG_DIR="$CHYREN_DIR/config"
STATE_DIR="$CHYREN_DIR/state"
SERVICES_DIR="$CHYREN_DIR/ops/services"

echo "─── MIGRATING OMEGA TO CHYREN ───"

# 1. Migrate /etc/omega (Config)
echo "→ Migrating /etc/omega..."
sudo cp -r /etc/omega/* "$CONFIG_DIR/" 2>/dev/null || true
# Rename files in config
for f in "$CONFIG_DIR"/omega-*; do
    if [[ -f "$f" ]]; then
        mv "$f" "${f//omega/chyren}"
    fi
done

# 2. Migrate /var/lib/omega (State)
echo "→ Migrating /var/lib/omega..."
sudo cp -r /var/lib/omega/* "$STATE_DIR/" 2>/dev/null || true
# These are mostly .json and .md files, names like ARCHITECTURE.json are fine.

# 3. Migrate ~/.omega (Vault)
echo "→ Migrating ~/.omega..."
mkdir -p "$STATE_DIR/vault"
cp -r /home/mega/.omega/* "$STATE_DIR/vault/" 2>/dev/null || true

# 4. Migrate Systemd Services
echo "→ Migrating Systemd Services..."
# Root service
sudo cp /omega-revenue-router.service "$SERVICES_DIR/chyren-revenue-router.service"
# Other services
for f in /etc/systemd/system/omega-*; do
    if [[ -f "$f" ]]; then
        target_name="${f##*/}"
        target_name="${target_name//omega/chyren}"
        sudo cp "$f" "$SERVICES_DIR/$target_name"
    fi
done

# 5. Global Rebrand of migrated files
echo "→ Rebranding migrated files..."
find "$CHYREN_DIR/ops/services" "$CHYREN_DIR/config" "$CHYREN_DIR/state" -type f -exec sed -i 's/omega/chyren/gI' {} +
find "$CHYREN_DIR/ops/services" "$CHYREN_DIR/config" "$CHYREN_DIR/state" -type f -exec sed -i 's/medulla/chyren-os\/kernel/gI' {} +
find "$CHYREN_DIR/ops/services" "$CHYREN_DIR/config" "$CHYREN_DIR/state" -type f -exec sed -i 's/OmegA-Sovereign/Chyren/gI' {} +

# 6. Update paths in service files to point to local repo
echo "→ Updating paths in service files..."
sed -i "s|/home/mega/Chyren/packages/brain|/home/mega/Chyren/gateway|g" "$SERVICES_DIR"/*.service 2>/dev/null || true
sed -i "s|/home/mega/Chyren/gAIng-brAin|/home/mega/Chyren/cortex|g" "$SERVICES_DIR"/*.service 2>/dev/null || true
sed -i "s|/etc/omega|/home/mega/Chyren/config|g" "$SERVICES_DIR"/*.service 2>/dev/null || true
sed -i "s|/var/lib/omega|/home/mega/Chyren/state|g" "$SERVICES_DIR"/*.service 2>/dev/null || true
sed -i "s|/home/mega/.omega|/home/mega/Chyren/state/vault|g" "$SERVICES_DIR"/*.service 2>/dev/null || true

echo "✓ Migration complete. Files are now organized in $CHYREN_DIR."
