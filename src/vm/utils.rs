pub fn swap_endian(original: [u8; 2]) -> u16 {
    original[1] as u16 + ((original[0] as u16) << 8)
}

pub fn sign_extend(x: u16, bit_count: u16) -> u16 {
    let mut y = x;
    if ((y >> (bit_count - 1)) & 1) > 0 {
        y |= 0xFFFF << bit_count;
    }
    y
}