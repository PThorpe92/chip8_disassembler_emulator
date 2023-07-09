
pub fn set(n: u8, x: usize) -> u8 {
    return n | (1 << x);
}

pub fn clear(n: u8, x: usize) -> u8 {
    return n & !(1 << x);
}
pub fn xor(n: u8, x: usize) -> u8 {
    return n ^ (1 << x);
}
pub fn swap(n: u8) -> u8 {
    return ((n & 0x0F) << 4) | ((n & 0xF0) >> 4);
}
pub fn get_bit(n: u8, x: usize) -> bool {
    return (n & (1 << x)) != 0;
}
