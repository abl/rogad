#!/bin/bash
set -euo pipefail

if [ "$*" == "--release" ]; then
    xargo build --target armv7-unknown-linux-gnueabihf --release
    ssh root@emuelec.local killall rogad || true
    du -h target/armv7-unknown-linux-gnueabihf/release/rogad
    scp target/armv7-unknown-linux-gnueabihf/release/rogad root@emuelec.local:~
    echo "---"
    ssh root@emuelec.local "/storage/rogad"
else
    cargo build --target=armv7-unknown-linux-gnueabihf
    ssh root@emuelec.local killall rogad || true
    scp target/armv7-unknown-linux-gnueabihf/debug/rogad root@emuelec.local:/storage/rogad
    echo "---"
    ssh root@emuelec.local "/storage/rogad"
fi