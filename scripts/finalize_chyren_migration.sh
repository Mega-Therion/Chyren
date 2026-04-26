#!/usr/bin/env bash
# Finalize Chyren Migration — System Switch-Over
# This script must be run with sudo.

set -euo pipefail

CHYREN_DIR="/home/mega/Chyren"
BACKUP_DIR="$CHYREN_DIR/archives/legacy_system_data/$(date +%Y%m%d_%H%M%S)"

echo "─── FINALIZING CHYREN MIGRATION ───"

# 1. Stop and Disable Legacy Omega Services
echo "→ Stopping and disabling legacy Omega services..."
OMEGA_SERVICES=$(systemctl list-units --all --type=service --no-legend "omega-*" | awk '{print $1}')
for svc in $OMEGA_SERVICES; do
    echo "  Stopping $svc..."
    systemctl stop "$svc" || true
    systemctl disable "$svc" || true
done

# 2. Backup Legacy Directories
echo "→ Backing up legacy directories to $BACKUP_DIR..."
mkdir -p "$BACKUP_DIR"
[ -d /etc/omega ] && mv /etc/omega "$BACKUP_DIR/etc_omega"
[ -d /var/lib/omega ] && mv /var/lib/omega "$BACKUP_DIR/var_lib_omega"
[ -d /home/mega/.omega ] && mv /home/mega/.omega "$BACKUP_DIR/home_omega"

# 3. Create Chyren System Links
echo "→ Creating Chyren system links..."
ln -sfn "$CHYREN_DIR/config" /etc/chyren
ln -sfn "$CHYREN_DIR/state" /var/lib/chyren

# 4. Install Chyren Service Units
echo "→ Installing Chyren systemd units..."
for svc_file in "$CHYREN_DIR/ops/services"/*.service; do
    svc_name=$(basename "$svc_file")
    echo "  Linking $svc_name..."
    ln -sfn "$svc_file" "/etc/systemd/system/$svc_name"
done

for timer_file in "$CHYREN_DIR/ops/services"/*.timer; do
    if [ -f "$timer_file" ]; then
        timer_name=$(basename "$timer_file")
        echo "  Linking $timer_name..."
        ln -sfn "$timer_file" "/etc/systemd/system/$timer_name"
    fi
done

# 5. Reload and Enable
echo "→ Reloading systemd and enabling new services..."
systemctl daemon-reload

for svc_file in "$CHYREN_DIR/ops/services"/*.service; do
    svc_name=$(basename "$svc_file")
    echo "  Enabling $svc_name..."
    systemctl enable "$svc_name"
done

# 6. Final Status
echo "✓ System migration to Chyren identity finalized."
echo "You can now start services individually or reboot."
echo "Example: sudo systemctl start chyren-brain"
