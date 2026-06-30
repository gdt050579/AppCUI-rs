use super::super::OutputBuffer;

pub(super) fn write(bytes: [u8; 8], output: &mut OutputBuffer) {
    let v = bytes[0];
    for i in 0..8 {
        let bit = (v >> (7 - i)) & 1;
        output.set(i, if bit != 0 { b'1' } else { b'0' });
    }
    output.set_len(8);
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::OutputBuffer;

    #[test]
    fn writes_eight_binary_digits_msb_first() {
        let mut output = OutputBuffer::new();
        write([0xAB, 0, 0, 0, 0, 0, 0, 0], &mut output);
        assert_eq!(output.as_slice(), b"10101011");

        output = OutputBuffer::new();
        write([0x00, 0, 0, 0, 0, 0, 0, 0], &mut output);
        assert_eq!(output.as_slice(), b"00000000");

        output = OutputBuffer::new();
        write([0xFF, 0, 0, 0, 0, 0, 0, 0], &mut output);
        assert_eq!(output.as_slice(), b"11111111");

        output = OutputBuffer::new();
        write([0x01, 0, 0, 0, 0, 0, 0, 0], &mut output);
        assert_eq!(output.as_slice(), b"00000001");
    }
}
