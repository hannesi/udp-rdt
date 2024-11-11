pub fn calculate_crc8_ccitt(array: &[u8]) -> u8 {
    let mut crc = 0u8;
    let poly = 0x07;

    for &byte in array {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ poly;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_calculate_crc_8(bytes: &[u8], expected: u8) {
        let result: u8 = calculate_crc8_ccitt(bytes);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_calculate_crc_8_0() {
        let bytes: [u8; 5] = [0x6b, 0x6f, 0x69, 0x72, 0x61];
        test_calculate_crc_8(&bytes, 0xc3);
    }

    #[test]
    fn test_calculate_crc_8_1() {
        let bytes: [u8; 7] = [0x68, 0x65, 0x76, 0x6f, 0x6e, 0x65, 0x6e];
        test_calculate_crc_8(&bytes, 0xcc);
    }

    #[test]
    fn test_calculate_crc_8_2() {
        let bytes: [u8; 5] = [0x6b, 0x69, 0x73, 0x73, 0x61];
        test_calculate_crc_8(&bytes, 0x87);
    }
}

