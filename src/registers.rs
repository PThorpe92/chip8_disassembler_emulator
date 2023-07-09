mod bits;
use bits::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)] 
pub struct Registers {
// Registers H and L can be used as a pair to form a 16 bit register
 // as can B and C, and D and E
    pub a: u8, // A is special because it is used in arithmetic operations
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8, //the H contains the most significant bits of the 
    pub l: u8, // referenced address. the L contains the least
    pub hl: u16,// when they are combined to make hl
    pub bc: u16, 
    pub de: u16, 
    pub flags: Flags,
    // Example. if the H contains 0x12 and the L contains 0x34,
    // then the referenced address is 0x1234
}
// The stack is an area of memory set aside by the programmer
// to store and retrieve data by stack operations (push, pop)
// The stack pointer is a special 16bit register that always 
// points to the top of the stack
impl Registers {
    //_______ GETTERS _________

    // Method to get the register pair AF
    // This works by shifting the value 8 bits to the left and then casting it to a u8
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | (self.flags.get() as u16)
    }

    // Method to get the register pair HL
    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | (self.l as u16)
    }

    // Method to get the register pair BC 
    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | (self.c as u16)
    }

    // Mothod to get the register pair DE
    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | (self.e as u16)
    }
    
    // _______ SETTERS _________
    pub fn set_af(&mut self, value: u16) { self.a = (value >> 8) as u8; self.flags.set((value & 0x00d5 | 0x002) as u8); }
    pub fn set_de(&mut self, value: u16) {
    // This works by shifting the value 8 bits to the right and then casting it to a u8
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00ff) as u8;
    }
    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00ff) as u8;
    }
    // Method to set register Pair HL with supplied Value
    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00ff) as u8;
    }
}
pub enum Flags {
    c(bool) = 0, // Carry
    p(bool) = 2, // Parity
    a(bool) = 4, // Auxillary Carry
    z(bool) = 6, // Zero
    s(bool) = 7, // Sign
}
impl Flags {
    pub fn new() -> Flags {
        Flags {
            z: false,
            s: false,
            p: false,
            cy: false,
            carry: false,
            auxcarry: false,
            parity: false,
        }
    }
    pub fn set(&mut self, flag: u8, value: bool) {
        match value {
            true => self.flag = bits::set(flag),
            false => self.flag = bits::clear(flag),
        }
    }
    pub fn get(&self, flag: u8) -> bool {
        match flag {}
            0 => self.z,
            1 => self.s,
            2 => self.p,
            3 => self.cy,
            4 => self.auxcarry,
            5 => self.parity,
            6 => self.carry,
            _ => false,
        }
        return bits::get_bit(self.flag, flag);
    }
}
