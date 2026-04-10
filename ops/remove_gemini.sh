#!/usr/bin/env bash

echo "== Removing unofficial Gemini Snap packages =="

# Remove snap packages
sudo snap remove gemini 2>/dev/null || true
sudo snap remove gemini-cli 2>/dev/null || true

echo "== Removing possible npm global installs =="
npm uninstall -g @google/gemini-cli gemini gemini-cli 2>/dev/null || true

echo "== Removing leftover binaries =="
sudo rm -f /usr/local/bin/gemini 2>/dev/null || true
sudo rm -f /usr/bin/gemini 2>/dev/null || true

echo "== Cleaning user directories =="
rm -rf ~/.gemini 2>/dev/null
rm -rf ~/.config/gemini 2>/dev/null
rm -rf ~/.cache/gemini 2>/dev/null
rm -rf ~/.local/share/gemini 2>/dev/null

echo "== Cleaning npm/nvm leftovers =="
rm -rf ~/.npm/*gemini* 2>/dev/null
rm -rf ~/.nvm/*/lib/node_modules/@google/gemini-cli 2>/dev/null

echo "== Removing any stray gemini symlinks =="
find ~/.nvm -type l -name "gemini" -delete 2>/dev/null

echo "== Checking for running processes =="
pkill -f gemini 2>/dev/null || true

echo "== Final verification =="
which gemini 2>/dev/null || echo "gemini not found"
command -v gemini 2>/dev/null || echo "no gemini in PATH"

echo "== Done. You are clean. =="
