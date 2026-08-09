#!/usr/bin/env bash

set -e

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if [[ $EUID -ne 0 ]]; then
    exec sudo "$0" "$@"
fi

echo "[+] Starting Bulwark engine..."
"$ROOT/engine/target/debug/engine" &

ENGINE_PID=$!

echo "[+] Starting Bulwark server..."
(
    cd "$ROOT/server"
    go run .
) &

SERVER_PID=$!

trap 'kill "$ENGINE_PID" "$SERVER_PID" 2>/dev/null' EXIT

wait
