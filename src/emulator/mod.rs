use num_derive::FromPrimitive;

use std::fmt;

mod util;
mod instruction;
pub use instruction::init_instructions;

#[derive(Debug, FromPrimitive)]
pub enum Register {
    EAX,
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
    RegisterCount,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self) // fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone)]
pub struct Emulator {
    pub registers: [u32; Register::RegisterCount as usize],
    pub eflags: u32,
    pub memory: Vec<u8>,
    pub eip: u32,
}

impl Emulator {
    pub fn create(size: usize, eip: u32, esp: u32) -> Emulator {
        Emulator {
            registers: {
                let mut regs = [0; Register::RegisterCount as usize];
                regs[Register::ESP as usize] = esp;
                regs
            },
            eflags: 0b10,
            memory: vec![0; size],
            eip: eip,
        }
    }
}