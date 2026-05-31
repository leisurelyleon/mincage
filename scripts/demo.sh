#!/usr/bin/env bash
# Build, generate a spec, and launch a sample sandboxed process.
#
# NOTE: launching a container requires privileges. If this fails with a
# permission error, re-run the final command with sudo:
#   sudo ./target/release/mincage run --spec spec.toml
set -euo pipefail

cargo build --release

if [ ! -f spec.toml ]; then
    echo "Generating spec.toml..."
    ./target/release/mincage spec > spec.toml
fi

echo "Launching sandboxed process (may require sudo)..."
./target/release/mincage run --spec spec.toml
