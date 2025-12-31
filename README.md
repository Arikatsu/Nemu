# Nemu

A Game Boy (DMG) emulator written in Rust. 

This repo contains:

- `nemu-core`: Core emulation logic as a Rust library + a built-in debugger binary + custom boot ROM source code
- `nemu-gui`: GUI for running ROMs (ignore for now, not actively maintained at this stage. Check out the debugger code in `nemu-core` instead)

## Getting Started

**Requirements:**

- [Rust toolchain](https://www.rust-lang.org/tools/install) (duh)
- [RGBDS toolchain](https://rgbds.gbdev.io/install)
- A computer (strongly recommended)

Clone with submodules:

```bash
git clone --recurse-submodules https://github.com/Arikatsu/nemu.git
```

Build the boot ROM:

```bash
cd nemu_core/bootrom
./build.sh # OR .\build.ps1 on Windows PowerShell
```

Run tests:
```bash
cargo test -p nemu-core --lib
```

Run with the debugger:
```bash
cargo run -p nemu-core --features debugger
```

## Current Status

Features implemented so far:

- [x] Full CPU instruction set and emulation
- [x] Bus and memory mapping (partial, will evolve as more features are implemented)
- [x] Timer
- [x] Interrupt handling
- [x] Debugger
- [x] Background/Window rendering and PPU mode switching
- [x] Sprite rendering
- [x] Joypad input
- [x] Custom Boot ROM (currently only does basic initialization, plan to show my own boot animation later)
- [ ] MBC cartridges
  - [X] ROM only
  - [x] MBC1 (ROM + RAM) (BATTERY SOON)
- [ ] Serial
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

- The project is still under active development and may contain bugs or incomplete behavior.
- Contributions, bug reports, and feedback are welcome.
- Yes, I have NOT come around to adding the HALT bug yet.