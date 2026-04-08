#!/bin/bash
# Chyren Docker Manager - Central orchestration for the OmegA Hub

set -e

COMMAND=$1
shift

function show_help() {
  echo "Usage: ./scripts/docker-manager.sh [command]"
  echo ""
  echo "Commands:"
  echo "  up        Start all services (detached)"
  echo "  down      Stop all services"
  echo "  build     Build OR Re-build all images"
  echo "  logs      View logs from all services (follow)"
  echo "  restart   Restart services"
  echo "  clean     Stop services and remove volumes (WIPE DATA)"
  echo ""
}

case $COMMAND in
  up)
    docker-compose up -d
    echo "[SYSTEM] Chyren Hub is coming online..."
    ;;
  down)
    docker-compose down
    ;;
  build)
    docker-compose build "$@"
    ;;
  logs)
    docker-compose logs -f "$@"
    ;;
  restart)
    docker-compose restart "$@"
    ;;
  clean)
    docker-compose down -v
    echo "[SYSTEM] Data wiped. Environment clean."
    ;;
  *)
    show_help
    ;;
esac
