mod bus;
mod cpu;
// mod traits;
mod timer;
mod ppu;
mod interrupts;
mod joypad;
mod mbc;

#[cfg(feature = "debugger")]
pub mod debugger;

#[cfg(feature = "debugger")]
pub use debugger::Debugger;
pub use joypad::JoypadButton;

#[derive(Debug)]
pub enum NemuError {
    InvalidRom(String),
}

impl std::fmt::Display for NemuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NemuError::InvalidRom(msg) => write!(f, "Invalid ROM: {}", msg),
        }
    }
}

pub struct Nemu {
    pub(crate) cpu: cpu::Cpu,
    pub(crate) bus: bus::Bus,
}

impl Default for Nemu {
    fn default() -> Self {
        Self {
            cpu: cpu::Cpu::new(),
            bus: bus::Bus::new(),
        }
    }
}

impl Nemu {
    pub fn reset(&mut self) {
        self.cpu.reset();
        self.bus.reset();
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step(&mut self.bus)
    }

    pub fn load_cartridge(&mut self, bytes: &[u8]) -> Result<(), NemuError> {
        self.bus.mbc = mbc::MbcType::new(bytes.to_vec())?;
        Ok(())
    }
    
    pub fn set_joypad(&mut self, input: JoypadButton, pressed: bool, is_direction: bool) {
        self.bus.joypad.set_joypad(input, pressed, is_direction);
    }

    pub fn has_frame(&mut self) -> bool {
        if self.bus.ppu.frame_ready {
            self.bus.ppu.frame_ready = false;
            true
        } else {
            false
        }
    }

    pub fn get_framebuffer(&mut self) -> &[u8; 160 * 144] {
        &self.bus.ppu.framebuffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_test_rom(path: &str) -> bool {
        let rom_data = std::fs::read(path).expect("Failed to read test ROM");
        let mut nemu = Nemu::default();
        nemu.load_cartridge(&rom_data).expect("Failed to load test ROM");
        let mut count = 10000;

        for _ in 0..10_000_000 {
            nemu.step();
            count -= 1;

            if count == 0 {
                count = 10000;

                let output = &nemu.bus.serial_output;
                if output.contains("Passed") {
                    return true;
                } else if output.contains("Failed") {
                    eprintln!("\x1b[32mSerial Output:\x1b[0m\n{}", output);
                    return false;
                }
            }
        }

        false
    }

    #[test]
    fn cpu_instrs_01() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/01-special.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_02() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/02-interrupts.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_03() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/03-op sp,hl.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_04() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/04-op r,imm.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_05() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/05-op rp.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_06() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/06-ld r,r.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_07() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_08() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/08-misc instrs.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_09() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/09-op r,r.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_10() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/10-bit ops.gb",
        );
        assert!(result);
    }

    #[test]
    fn cpu_instrs_11() {
        let result = run_test_rom(
            "../tests/cpu_instrs/individual/11-op a,(hl).gb",
        );
        assert!(result);
    }

    #[test]
    fn instr_timing() {
        let result = run_test_rom(
            "../tests/instr_timing/instr_timing.gb",
        );
        assert!(result);
    }

    #[test]
    fn mem_timing_01() {
        let result = run_test_rom(
            "../tests/mem_timing/individual/01-read_timing.gb",
        );
        assert!(result);
    }

    #[test]
    fn mem_timing_02() {
        let result = run_test_rom(
            "../tests/mem_timing/individual/02-write_timing.gb",
        );
        assert!(result);
    }

    #[test]
    fn mem_timing_03() {
        let result = run_test_rom(
            "../tests/mem_timing/individual/03-modify_timing.gb",
        );
        assert!(result);
    }
}