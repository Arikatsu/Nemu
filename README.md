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

(no tests exist yet lol, getting some basic ops down first)

## Future Plans

- Complete CPU implementation
- Implement PPU and APU
- Add save state support
- Compile for multiple platforms (WASM, desktop, etc.)
