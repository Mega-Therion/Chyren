#!/bin/bash
# Chyren Docker Manager - Central orchestration for the Chyren Hub

set -e

COMMAND=$1
shift

# Path to the Sovereign Compose file
COMPOSE_FILE="chyren_workspace/workspace/Chyren-Next/docker-compose.yml"

function show_help() {
  echo "Usage: ./scripts/docker-manager.sh [command]"
  echo ""
  echo "Commands:"
  echo "  up        Start all services (detached)"
  echo "  down      Stop all services"
  echo "  build     Build OR Re-build all images"
  echo "  logs      View logs from all services (follow)"
  echo "  status    Check service health"
  echo "  restart   Restart services"
  echo "  clean     Stop services and remove volumes (WIPE DATA)"
  echo ""
}

case $COMMAND in
  up)
    docker-compose -f $COMPOSE_FILE up -d
    echo "[SYSTEM] Chyren Hub is coming online..."
    ;;
  down)
    docker-compose -f $COMPOSE_FILE down
    ;;
  build)
    docker-compose -f $COMPOSE_FILE build "$@"
    ;;
  logs)
    docker-compose -f $COMPOSE_FILE logs -f "$@"
    ;;
  status)
    docker-compose -f $COMPOSE_FILE ps
    ;;
  restart)
    docker-compose -f $COMPOSE_FILE restart "$@"
    ;;
  clean)
    docker-compose -f $COMPOSE_FILE down -v
    echo "[SYSTEM] Data wiped. Environment clean."
    ;;
  *)
    show_help
    ;;
esac
