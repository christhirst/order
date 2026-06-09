#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")"

export LD_LIBRARY_PATH="/home/c/workspace/order/instantclient_23_26"
env
workspace_root="$(cd "$PWD/.." && pwd)"
binary="$workspace_root/target/debug/order"

if [[ -x "$binary" ]]; then
    exec "$binary" "$@"
fi

exec cargo run -- "$@"
