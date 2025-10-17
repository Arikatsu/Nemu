use nemu_core::Nemu;

fn main() {
    let mut nemu = Nemu::with_rom("tests/cpu_instrs/individual/05-op rp.gb").expect("Failed to load ROM");

    let mut cycles = 0u64;
    let max_cycles = 10_000_000u64;

    while cycles < max_cycles {
        let c = nemu.step() as u64;
        cycles += c;
    }
}