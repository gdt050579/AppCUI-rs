use super::super::OutputBuffer;

pub(super) fn write(bytes: [u8;8], output: &mut OutputBuffer) {
    let v = bytes[0] as u32;
    output.set(0, ((v >> 6) & 0x07) as u8 + b'0');
    output.set(1, ((v >> 3) & 0x07) as u8 + b'0');
    output.set(2, (v & 0x07) as u8 + b'0');
    output.set_len(3);
}
pub(super) fn convert_to_bytes(text: &str) -> ([u8; 8], u8) {
    todo!()
}