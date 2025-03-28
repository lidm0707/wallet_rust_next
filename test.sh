#!/bin/bash

# Exit script on errors
set -e

# Paths
DB_DIR="DB"
FRONTEND_DIR="rust_next"
BACKEND_DIR="tranfer_transaction"

# Load environment variables from the backend directory
ENV_FILE="$BACKEND_DIR/.env_for_bash"

if [ -f $ENV_FILE ]; then
  export $(cat $ENV_FILE | xargs)
else
  echo "Error: .env file not found in $BACKEND_DIR. Ensure necessary environment variables are set."
  exit 1
fi

# Database and server ports
DB_PORT=${DB_PORT:-5432}  # Default to 5432 if not set
SERVER_PORT=${SERVER_PORT:-8080}  # Default to 8080 if not set

# Log function with timestamp
log() {
  echo "[$(date +'%Y-%m-%d %H:%M:%S')] $1"
}

# Ping check function
ping_service() {
  local HOST=$1
  local PORT=$2
  log "Pinging $HOST:$PORT..."
  while ! nc -z $HOST $PORT; do
    log "Waiting for $HOST:$PORT to be available..."
    sleep 2
  done
  log "$HOST:$PORT is available!"
}

# Start containers using docker-compose
start_containers() {
  log "Starting containers for DB, Frontend, and Backend..."

  # Ensure docker-compose.yml exists in the DB directory
  if [ ! -f "$DB_DIR/docker-compose.yml" ]; then
    echo "Error: $DB_DIR/docker-compose.yml file not found."
    exit 1
  fi

  # Start the database container using docker-compose
  docker-compose -f $DB_DIR/docker-compose.yml up -d

  # Wait for the database to be ready
  ping_service "localhost" $DB_PORT

  # Build and run the frontend container using Docker (you might want to build this if not already)
  log "Building and running the frontend container..."
  # docker build -t rust_next_image $FRONTEND_DIR
  docker run -d --name rust_frontend_container -p 3000:3000 rust_next_image

  # Build and run the backend container using Docker
  log "Building and running the backend container..."
  # docker build -t rust_backend_image $BACKEND_DIR
  docker run -d --name rust_backend_container --network db_my_network -p $SERVER_PORT:$SERVER_PORT --env-file $ENV_FILE rust_backend_image
}

# Wait for services (ping check)
wait_for_services() {
  log "Waiting for services to be ready..."
  ping_service "localhost" $DB_PORT
  ping_service "localhost" $SERVER_PORT
}

# Run Playwright tests
run_playwright_tests() {
  log "Running Playwright tests..."
  
  # Ensure you are in the correct directory where tests are located
  cd test

  # Run Playwright tests, specify path if necessary (adjust the pattern to match your test files)
  npx playwright test --project=chromium --config=playwright.config.ts
  
  if [ $? -eq 0 ]; then
    log "Playwright tests passed!"
  else
    log "Playwright tests failed!" >&2
    exit 1
  fi
  cd ../..
}

# Stop containers using docker-compose
stop_containers() {
  log "Stopping all containers..."

  # Ensure docker-compose.yml exists in the DB directory

  # Stop containers using docker-compose
  
  # Gracefully stop containers before removing
  docker stop rust_frontend_container rust_backend_container new_db || true

  # Force remove the containers (if they didn't stop correctly)
  docker rm -f rust_frontend_container rust_backend_container new_db || true
}

# Ensure cleanup on script exit
trap stop_containers EXIT

# Main workflow
main() {
  start_containers
  wait_for_services
  run_playwright_tests
  stop_containers
}

# Execute the workflow
main
