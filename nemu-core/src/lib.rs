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

        for _ in 0..100_000_000 {
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

        eprintln!("\x1b[31mTest ROM timed out without a result.\x1b[0m");
        false
    }

    #[test]
    fn cpu_instrs() {
        let result = run_test_rom(
            "../tests/cpu_instrs/cpu_instrs.gb",
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
    fn mem_timing() {
        let result = run_test_rom(
           "../tests/mem_timing/mem_timing.gb",
        );
        assert!(result);
    }
}