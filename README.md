# Nemu

A Game Boy emulator in Rust. This repo contains:

- `nemu-core`: CPU/memory core
- `nemu-gui`: binary to run ROMs with visual output

## Getting Started

Clone with submodules:

```bash
git clone --recurse-submodules https://github.com/Arikatsu/nemu.git
```

Run tests:
```bash
cargo test -p nemu-core --lib
```

## Future Plans

- Implement PPU
- Implement Joypad
- Implement Serial
- Add save state support
- Compile for multiple platforms (WASM, desktop, etc.)
