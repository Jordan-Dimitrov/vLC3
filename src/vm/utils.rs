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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_endian() {
        assert_eq!(swap_endian([0x01, 0x02]),  0x0102);
        assert_eq!(swap_endian([0xFF, 0xEE]),  0xFFEE);
    }

    #[test]
    fn test_sign_extend() {
        assert_eq!(sign_extend(0x80, 8), 0xFF80);
        assert_eq!(sign_extend(0xFF, 8), 0xFFFF);

        assert_eq!(sign_extend(0x8, 4), 0xFFF8);
    }
}