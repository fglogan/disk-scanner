#!/bin/bash

# Kill any existing instances
echo "Stopping any existing instances..."
pkill -f "disk-bloat-scanner.*tauri"
pkill -f "disk-bloat-scanner.*vite"
sleep 2

# Navigate to project directory
cd "$(dirname "$0")"

# Start Tauri dev
echo "Starting Tauri dev server..."
npx tauri dev
