mod cpu;
mod display;
mod input;
mod asm;
mod registers;
mod bits;
use crate::cpu::CPU;
use crate::asm::parse_rom;



pub struct Emulator {
    pub cpu: CPU,
    pub memory: Memory,
    pub registers: Registers,
    pub input: Input,
    pub display: Display,
    pub stack: Stack,
    pub interrupt: Interrupt,
    pub timer: Timer,
    }
