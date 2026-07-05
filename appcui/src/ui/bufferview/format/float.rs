use super::FloatFormat;
use super::super::OutputBuffer;
use super::ValidateResult;

// Field widths — must match display_chars exactly.
// f32: " D.DDe±EE"  = sign(1)+digit(1)+dot(1)+2mant+e(1)+expsign(1)+2exp = 9
// f64: " D.DDDDDDDDDDDDDDDe±EEE" = sign(1)+1+1+15mant+e(1)+1+3exp = 22
const F32_MANT: usize = 2;
const F32_EXP: usize = 2;   // f32 |exp| <= 38
const F32_WIDTH: usize = 3 + F32_MANT + 2 + F32_EXP; // sign+digit+dot + mant + e+expsign + exp
const F64_MANT: usize = 15;
const F64_EXP: usize = 3;   // f64 |exp| <= 308
const F64_WIDTH: usize = 3 + F64_MANT + 2 + F64_EXP;

#[inline(always)]
pub(super) fn bytes_count(format: FloatFormat) -> u8 {
    match format {
        FloatFormat::Scientific32 => 4,
        FloatFormat::Scientific64 => 8,
    }
}

fn value_from_bytes(bytes: [u8; 8], format: FloatFormat) -> f64 {
    match format {
        // NOTE: le, not ne — file endianness is defined, host endianness is not.
        FloatFormat::Scientific32 => f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64,
        FloatFormat::Scientific64 => f64::from_le_bytes(bytes),
    }
}

pub(super) fn display_chars(format: FloatFormat) -> u32 {
    match format {
        FloatFormat::Scientific32 => F32_WIDTH as u32, // 9
        FloatFormat::Scientific64 => F64_WIDTH as u32, // 22
    }
}

pub(super) fn write(bytes: [u8; 8], format: FloatFormat, output: &mut OutputBuffer) {
    let (mant, exp_digits, width) = match format {
        FloatFormat::Scientific32 => (F32_MANT, F32_EXP, F32_WIDTH),
        FloatFormat::Scientific64 => (F64_MANT, F64_EXP, F64_WIDTH),
    };
    let v = value_from_bytes(bytes, format);

    // Non-finite: pad to exact width, right-aligned.
    if !v.is_finite() {
        let s: &str = if v.is_nan() { "NaN" } else if v > 0.0 { "inf" } else { "-inf" };
        emit_padded(s.as_bytes(), width, output);
        return;
    }

    // Build "D.DDe±EE" with a fixed sign slot and zero-padded exponent.
    // Rust's {:e} gives shortest exponent and no sign slot, so do it by hand.
    let neg = v.is_sign_negative();
    let a = v.abs();

    // Decompose into mantissa in [1,10) and base-10 exponent.
    // (a == 0 handled specially: exponent 0, mantissa 0.)
    let (mantissa, exp): (f64, i32) = if a == 0.0 {
        (0.0, 0)
    } else {
        let e = a.log10().floor() as i32;
        (a / 10f64.powi(e), e)
    };

    // Round mantissa to `mant` decimals; rounding can push it to 10.0 → renormalize.
    let scale = 10f64.powi(mant as i32);
    let mut m = (mantissa * scale).round() / scale;
    let mut e = exp;
    if m >= 10.0 {
        m /= 10.0;
        e += 1;
    }

    let mut buf = [0u8; 32]; // scratch, >= max width
    let mut n = 0usize;

    // sign slot: '-' or ' ' (space keeps columns aligned)
    buf[n] = if neg { b'-' } else { b'+' }; n += 1;

    // leading digit
    let lead = m.trunc() as u8; // 0..=9
    buf[n] = b'0' + lead; n += 1;
    buf[n] = b'.'; n += 1;

    // fractional mantissa digits
    let mut frac = m - (lead as f64);
    for _ in 0..mant {
        frac *= 10.0;
        let d = frac.trunc() as u8; // 0..=9
        buf[n] = b'0' + d; n += 1;
        frac -= d as f64;
    }

    // 'e', exponent sign, zero-padded exponent
    buf[n] = b'e'; n += 1;
    buf[n] = if e < 0 { b'-' } else { b'+' }; n += 1;
    let mut ae = e.unsigned_abs();
    // write exp_digits, zero-padded, most-significant first
    let start = n;
    for i in 0..exp_digits {
        let place = 10u32.pow((exp_digits - 1 - i) as u32);
        let d = ((ae / place) % 10) as u8;
        buf[start + i] = b'0' + d; n += 1;
    }
    let _ = &mut ae;

    debug_assert_eq!(n, width, "float field width mismatch");
    emit_padded(&buf[..n], width, output);
}

// Copy `src` into output, right-aligned in `width` (space-padded on the left).
fn emit_padded(src: &[u8], width: usize, output: &mut OutputBuffer) {
    let src = &src[..src.len().min(width)];
    let pad = width - src.len();
    let mut i = 0;
    while i < pad { output.set(i, b' '); i += 1; }
    for (j, &b) in src.iter().enumerate() {
        output.set(pad + j, b);
    }
    output.set_len(width as u8);
}

pub(super) fn validate(text: &str, format: FloatFormat) -> ValidateResult {
    // Permissive prefix check: accept anything that could still become a valid float.
    // FieldEdit model — the user is mid-typing, so "3.", "-", "1e", "1.2e-" are all OK.
    if is_float_prefix(text) {
        ValidateResult::Valid
    } else {
        ValidateResult::FormatError
    }
}

pub(super) fn convert_to_bytes(text: &str, format: FloatFormat) -> ([u8; 8], u8) {
    let t = text.trim();
    let mut out = [0u8; 8];
    match format {
        FloatFormat::Scientific32 => match t.parse::<f32>() {
            Ok(v) => { out[..4].copy_from_slice(&v.to_le_bytes()); (out, 4) }
            Err(_) => (out, 0), // commit fails; validate should have caught most
        },
        FloatFormat::Scientific64 => match t.parse::<f64>() {
            Ok(v) => { out.copy_from_slice(&v.to_le_bytes()); (out, 8) }
            Err(_) => (out, 0),
        },
    }
}

// Accept prefixes of a valid float: optional sign, digits, one dot, one 'e'/'E'
// with optional exponent sign, plus the special words inf/nan (and their prefixes).
fn is_float_prefix(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() { return true; } // empty is a valid prefix (still typing)

    // allow "inf"/"nan"/"-inf"/"+inf" and their prefixes, case-insensitive
    let lower = t.trim_start_matches(['+', '-']).to_ascii_lowercase();
    if "inf".starts_with(&lower) || "nan".starts_with(&lower) || "infinity".starts_with(&lower) {
        return true;
    }

    let bytes = t.as_bytes();
    let mut i = 0;
    if bytes[i] == b'+' || bytes[i] == b'-' { i += 1; }
    let mut seen_dot = false;
    let mut seen_e = false;
    let mut seen_digit = false;
    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => seen_digit = true,
            b'.' if !seen_dot && !seen_e => seen_dot = true,
            b'e' | b'E' if !seen_e && seen_digit => {
                seen_e = true;
                // optional exponent sign immediately after
                if i + 1 < bytes.len() && (bytes[i + 1] == b'+' || bytes[i + 1] == b'-') {
                    i += 1;
                }
            }
            _ => return false, // definitively not a float prefix
        }
        i += 1;
    }
    true
}