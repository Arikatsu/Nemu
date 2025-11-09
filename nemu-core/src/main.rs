use nemu_core::Nemu;

fn main() {
    let mut nemu = Nemu::with_rom("tests/cpu_instrs/individual/02-interrupts.gb").expect("Failed to load ROM");

    for _ in 0..500_000 {
        nemu.step();
    }
}