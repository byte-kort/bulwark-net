#!/usr/bin/env bash

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ $EUID -ne 0 ]]; then
    exec sudo "$0" "$@"
fi

echo "[+] Starting Bulwark engine..."
(
    cd "$ROOT/engine"
    cargo run
) &

echo "[+] Starting Bulwark server..."
(
    cd "$ROOT/server"
    go run .
) &

ENGINE_PID=$!

SERVER_PID=$!

trap 'kill "$ENGINE_PID" "$SERVER_PID" 2>/dev/null' EXIT

echo "[+] Bulwark started"
echo "[+] Engine PID: $ENGINE_PID"
echo "[+] Server PID: $SERVER_PID"

wait
