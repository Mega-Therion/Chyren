#!/usr/bin/env bash
# launch-voice-engines.sh
# Deploys local Whisper.cpp and Piper TTS instances using Docker

set -euo pipefail

echo "=================================================="
echo "    Deploying Chyren Local Voice Engines..."
echo "=================================================="

# Check for Docker
if ! command -v docker &> /dev/null; then
  echo "Error: Docker is required but not installed."
  exit 1
fi

echo "1) Deploying Whisper.cpp (STT) on port 8178..."
# Runs Whisper.cpp server using the base English model
if [ "$(docker ps -q -f name=chyren-whisper)" ]; then
    echo "   -> Whisper container is already running."
else
    docker run -d --rm \
      --name chyren-whisper \
      -p 8178:8080 \
      ghcr.io/ggerganov/whisper.cpp:main \
      --model base.en --host 0.0.0.0 --port 8080
    echo "   -> Whisper.cpp deployed."
fi

echo "2) Deploying Piper (TTS) on port 5030..."
# Runs a community Piper TTS server (e.g. lscr.io/linuxserver/piper)
if [ "$(docker ps -q -f name=chyren-piper)" ]; then
    echo "   -> Piper container is already running."
else
    # Create models directory if it doesn't exist
    mkdir -p "$PWD/models"
    
    docker run -d --rm \
      --name chyren-piper \
      -p 5030:5000 \
      -e TZ=Etc/UTC \
      -e PIPER_VOICE=en_US-lessac-medium \
      -v "$PWD/models:/config/models" \
      lscr.io/linuxserver/piper:latest
    echo "   -> Piper TTS deployed."
fi

echo "=================================================="
echo " Voice Engines are spinning up!"
echo " - Whisper.cpp : http://localhost:8178"
echo " - Piper TTS   : http://localhost:5030"
echo "=================================================="
