#!/usr/bin/env bash
set -e

mkdir -p build

rgbasm -o build/dmg_boot.o dmg_boot.asm
rgblink -x -o build/dmg_boot.bin build/dmg_boot.o

echo "Build succeeded. Output: build/dmg_boot.bin"