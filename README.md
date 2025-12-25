# Nemu

A Game Boy (DMG) emulator in Rust. 

This repo contains:

- `nemu-core`: Core emulation logic as a Rust library + a built-in debugger binary
- `nemu-gui`: GUI for running ROMs (ignore for now, not actively maintained at this stage. Check out the debugger code in `nemu-core` instead)
- `bootrom`: Custom Boot ROM source code in RGBDS assembly

## Getting Started

**Requirements:**

Core:
- Rust >= v1.85.0

Boot ROM:
- Make
- [RGBDS toolchain](https://rgbds.gbdev.io/install)

Clone with submodules:

```bash
git clone --recurse-submodules https://github.com/Arikatsu/nemu.git
```

Run tests:
```bash
cargo test -p nemu-core --lib
```

Run the interactive debugger:
```bash
cargo run -p nemu-core --features debugger
```

## Current Status

Nemu is currently able to boot and run simple ROMs that do not require MBC banking.

Features implemented so far:

- [x] Full CPU instruction set and emulation
- [x] Bus and memory mapping (partial, will evolve as more features are implemented)
- [x] Timer
- [x] Interrupt handling
- [x] Debugger (partial, missing breakpoints specially. WIP)
- [x] Background/Window rendering and PPU mode switching
- [x] Sprite rendering
- [x] Joypad input
- [x] Custom Boot ROM (in progress, currently only does basic initialization)
- [ ] MBC cartridges
- [ ] Sound
- [ ] Save states
- [ ] GUI for running ROMs (nemu-gui)

**List items may be updated or even changed as development progresses and does not indicate a strict roadmap.**

## Passing Tests

- Blargg's individual CPU instruction tests
- Blargg's CPU and Memory instruction timing tests
- `dmg-acid2` test ROM

(need to test more lol)

## Notes

- The code is still a work in progress and may contain bugs or incomplete features. (Looking at you Halt Bug!!!!)
- Contributions and feedback are welcome!