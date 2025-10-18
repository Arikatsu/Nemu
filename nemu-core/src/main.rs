use nemu_core::Nemu;

fn main() {
    let mut nemu = Nemu::with_rom("tests/cpu_instrs/individual/11-op a,(hl).gb").expect("Failed to load ROM");

    for _ in 0..10_000_000 {
        nemu.step();
    }
}