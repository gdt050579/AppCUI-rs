use super::super::BytesCount;
use super::super::OutputBuffer;
use super::ValidateResult;


pub(super) fn write(bytes: [u8;8], bytes_count: BytesCount, output: &mut OutputBuffer) {
    todo!()
}
pub(super) fn validate(text: &str, bytes_count: BytesCount) -> ValidateResult {
    todo!()
}
pub(super) fn convert_to_bytes(text: &str, bytes_count: BytesCount) -> ([u8; 8], u8) {
    todo!()
}
pub(super) fn display_chars(bytes_count: BytesCount) -> u32 {
    todo!()
}