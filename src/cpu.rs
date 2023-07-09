use std::cell::RefCell;
use std::rc::Rc;
mod bits;
use crate::bits::*;

// Struct to emulate an Intel 8080 CPU.
#[derive(Debug, Clone, Copy)]
pub struct CPU {
    registers: Registers, // 8-bit registers. A, B, C, D, E, F, (H, L  can be combined to form HL)
    index: u16, // Index register.
    pc: u16,  // Program counter representing the address of the next instruction.
    memory: [u8; 4096], // 4K of memory.
    stack: Stack, // Stack + stack pointer.
    delay_timer: u8, // delay timer: 60 Hz
    sound_timer: u8, // Timers count at 60 Hz.
    keypad: [u8; 16], // 16-key hexadecimal keypad
    display: [u8; 64 * 32], // 64 x 32 monochrome display.
    draw_flag: bool, // Flag to indicate if the screen should be redrawn.
    opcode: u16, // Current opcode.
}
impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            index: 0,
            pc: 0,
            memory: [0; 4096],
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            keypad: [0; 16],
            display: [0; 64 * 32],
            draw_flag: false,
            opcode: 0x00,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Stack {
     stack: [u16; 16],
     pointer: u16,
}
impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; 16],
            pointer: 0,
        }
    }
    pub fn push(&mut self, value: u16) {
        self.stack[self.pointer as usize] = value;
        self.pointer += 1;
    }
    pub fn pop(&mut self) -> u16 {
        self.pointer -= 1;
        self.stack[self.pointer as usize]
    }
    pub fn reset(&mut self) {
        self.pointer = 0;
        self.stack = [0; 16];
    }
    pub fn get(&self) -> u16 {
        self.stack[self.pointer as usize]
    }
    pub fn peek(&self) -> u16 {
        if self.stack.is_empty() {
            return 0;
        }
        if self.stack.len() > 1 {
            return self.stack[(self.pointer - 1) as usize]
        } else {
           return self.stack[0]
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Memory {
    memory: [u8; 4096],
}
impl Memory {
    pub fn new() -> Memory {
        Memory {
            memory: [0; 4096],
        }
    }
    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }   
}

