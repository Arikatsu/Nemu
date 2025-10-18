mod load;
mod alu;
mod bits;
mod control;
mod stack;
mod misc;

pub(in crate::cpu) use load::*;
pub(in crate::cpu) use alu::*;
pub(in crate::cpu) use bits::*;
pub(in crate::cpu) use control::*;
pub(in crate::cpu) use stack::*;
pub(in crate::cpu) use misc::*;

pub(in crate::cpu) struct InstructionContext<'a> {
    pub(in crate::cpu) cpu: &'a mut crate::cpu::Cpu,
    pub(in crate::cpu) memory: &'a mut crate::memory::Memory,
}