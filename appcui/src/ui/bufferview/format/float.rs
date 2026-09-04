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

// E4M3 (OCP FP8): max finite 240, 240 distinct finite values — 3 int + 3 dec digits suffice.
// "+240.000" = sign(1)+int(3)+dot(1)+dec(3) = 8
const E4M3_INT: usize = 3;
const E4M3_DEC: usize = 3;
const E4M3_WIDTH: usize = 1 + E4M3_INT + 1 + E4M3_DEC;

// E5M2 (OCP FP8): max finite 57344, 248 distinct finite values — 5 int + 6 dec digits suffice.
// "+57344.000000" = sign(1)+int(5)+dot(1)+dec(6) = 13
const E5M2_INT: usize = 5;
const E5M2_DEC: usize = 6;
const E5M2_WIDTH: usize = 1 + E5M2_INT + 1 + E5M2_DEC;

#[inline(always)]
pub(super) fn bytes_count(format: FloatFormat) -> u8 {
    match format {
        FloatFormat::Scientific32 => 4,
        FloatFormat::Scientific64 => 8,
        FloatFormat::E4M3 | FloatFormat::E5M2 => 1,
    }
}

fn value_from_bytes(bytes: [u8; 8], format: FloatFormat) -> f64 {
    match format {
        // NOTE: le, not ne — file endianness is defined, host endianness is not.
        FloatFormat::Scientific32 => {
            f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as f64
        }
        FloatFormat::Scientific64 => f64::from_le_bytes(bytes),
        FloatFormat::E4M3 => e4m3_to_f64(bytes[0]),
        FloatFormat::E5M2 => e5m2_to_f64(bytes[0]),
    }
}

fn e4m3_to_f64(byte: u8) -> f64 {
    let sign = byte >> 7;
    let exp = (byte >> 3) & 0xF;
    let frac = byte & 0x7;
    let v = match exp {
        0 if frac == 0 => 0.0,
        0 => 2f64.powi(-6) * (frac as f64 / 8.0),
        15 => f64::NAN,
        _ => 2f64.powi(exp as i32 - 7) * (1.0 + frac as f64 / 8.0),
    };
    if sign != 0 { -v } else { v }
}

fn e5m2_to_f64(byte: u8) -> f64 {
    let sign = byte >> 7;
    let exp = (byte >> 2) & 0x1F;
    let frac = byte & 0x3;
    let v = match exp {
        0 if frac == 0 => 0.0,
        0 => 2f64.powi(-14) * (frac as f64 / 4.0),
        31 if frac == 0 => f64::INFINITY,
        31 => f64::NAN,
        _ => 2f64.powi(exp as i32 - 15) * (1.0 + frac as f64 / 4.0),
    };
    if sign != 0 { -v } else { v }
}

fn f64_to_e4m3(v: f64) -> u8 {
    if v.is_nan() || !v.is_finite() {
        return 0x7F;
    }
    if v == 0.0 {
        return if v.is_sign_negative() { 0x80 } else { 0x00 };
    }
    nearest_fp8_byte(v, e4m3_to_f64, |b| e4m3_to_f64(b).is_finite())
}

fn f64_to_e5m2(v: f64) -> u8 {
    if v.is_nan() {
        return 0x7F;
    }
    if v.is_infinite() {
        return if v.is_sign_negative() { 0xFC } else { 0x7C };
    }
    if v == 0.0 {
        return if v.is_sign_negative() { 0x80 } else { 0x00 };
    }
    nearest_fp8_byte(v, e5m2_to_f64, |b| e5m2_to_f64(b).is_finite())
}

fn nearest_fp8_byte(v: f64, decode: fn(u8) -> f64, is_finite: fn(u8) -> bool) -> u8 {
    let mut best = 0u8;
    let mut best_err = f64::INFINITY;
    for b in 0..=255u8 {
        if !is_finite(b) {
            continue;
        }
        let err = (decode(b) - v).abs();
        if err < best_err {
            best_err = err;
            best = b;
        }
    }
    best
}

pub(super) fn display_chars(format: FloatFormat) -> u32 {
    match format {
        FloatFormat::Scientific32 => F32_WIDTH as u32, // 9
        FloatFormat::Scientific64 => F64_WIDTH as u32, // 22
        FloatFormat::E4M3 => E4M3_WIDTH as u32,         // 8
        FloatFormat::E5M2 => E5M2_WIDTH as u32,         // 13
    }
}

pub(super) fn write(bytes: [u8; 8], format: FloatFormat, output: &mut OutputBuffer) {
    match format {
        FloatFormat::Scientific32 | FloatFormat::Scientific64 => {
            write_scientific(bytes, format, output);
        }
        FloatFormat::E4M3 => {
            write_fixed_decimal(value_from_bytes(bytes, format), E4M3_INT, E4M3_DEC, E4M3_WIDTH, output);
        }
        FloatFormat::E5M2 => {
            write_fixed_decimal(value_from_bytes(bytes, format), E5M2_INT, E5M2_DEC, E5M2_WIDTH, output);
        }
    }
}

fn write_scientific(bytes: [u8; 8], format: FloatFormat, output: &mut OutputBuffer) {
    let (mant, exp_digits, width) = match format {
        FloatFormat::Scientific32 => (F32_MANT, F32_EXP, F32_WIDTH),
        FloatFormat::Scientific64 => (F64_MANT, F64_EXP, F64_WIDTH),
        _ => unreachable!(),
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
    buf[n] = if neg { b'-' } else { b'+' };
    n += 1;

    // leading digit
    let lead = m.trunc() as u8; // 0..=9
    buf[n] = b'0' + lead;
    n += 1;
    buf[n] = b'.';
    n += 1;

    // fractional mantissa digits
    let mut frac = m - (lead as f64);
    for _ in 0..mant {
        frac *= 10.0;
        let d = frac.trunc() as u8; // 0..=9
        buf[n] = b'0' + d;
        n += 1;
        frac -= d as f64;
    }

    // 'e', exponent sign, zero-padded exponent
    buf[n] = b'e';
    n += 1;
    buf[n] = if e < 0 { b'-' } else { b'+' };
    n += 1;
    let mut ae = e.unsigned_abs();
    // write exp_digits, zero-padded, most-significant first
    let start = n;
    for i in 0..exp_digits {
        let place = 10u32.pow((exp_digits - 1 - i) as u32);
        let d = ((ae / place) % 10) as u8;
        buf[start + i] = b'0' + d;
        n += 1;
    }
    let _ = &mut ae;

    debug_assert_eq!(n, width, "float field width mismatch");
    emit_padded(&buf[..n], width, output);
}

// Fixed "+III.DDD" layout: sign, zero-padded integer, dot, fixed decimal digits.
fn write_fixed_decimal(v: f64, int_digits: usize, dec_digits: usize, width: usize, output: &mut OutputBuffer) {
    if !v.is_finite() {
        let s: &str = if v.is_nan() { "NaN" } else if v > 0.0 { "inf" } else { "-inf" };
        emit_padded(s.as_bytes(), width, output);
        return;
    }

    let neg = v.is_sign_negative();
    let a = v.abs();
    let scale = 10f64.powi(dec_digits as i32);
    let rounded = (a * scale).round() / scale;
    let lead = rounded.trunc() as u64;
    let mut frac = rounded - lead as f64;

    let mut buf = [0u8; 32];
    let mut n = 0usize;

    buf[n] = if neg { b'-' } else { b'+' };
    n += 1;

    let int_start = n;
    for i in 0..int_digits {
        let place = 10u64.pow((int_digits - 1 - i) as u32);
        let d = ((lead / place) % 10) as u8;
        buf[int_start + i] = b'0' + d;
        n += 1;
    }

    buf[n] = b'.';
    n += 1;

    for _ in 0..dec_digits {
        frac *= 10.0;
        let d = frac.trunc() as u8;
        buf[n] = b'0' + d;
        n += 1;
        frac -= d as f64;
    }

    debug_assert_eq!(n, width, "fp8 field width mismatch");
    emit_padded(&buf[..n], width, output);
}

// Copy `src` into output, right-aligned in `width` (space-padded on the left).
fn emit_padded(src: &[u8], width: usize, output: &mut OutputBuffer) {
    let src = &src[..src.len().min(width)];
    let pad = width - src.len();
    let mut i = 0;
    while i < pad {
        output.set(i, b' ');
        i += 1;
    }
    for (j, &b) in src.iter().enumerate() {
        output.set(pad + j, b);
    }
    output.set_len(width as u8);
}

pub(super) fn validate(text: &str, format: FloatFormat) -> ValidateResult {
    let ok = match format {
        FloatFormat::Scientific32 | FloatFormat::Scientific64 => is_float_prefix(text),
        FloatFormat::E4M3 | FloatFormat::E5M2 => is_fixed_decimal_prefix(text),
    };
    if ok {
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
            Ok(v) => {
                out[..4].copy_from_slice(&v.to_le_bytes());
                (out, 4)
            }
            Err(_) => (out, 0),
        },
        FloatFormat::Scientific64 => match t.parse::<f64>() {
            Ok(v) => {
                out.copy_from_slice(&v.to_le_bytes());
                (out, 8)
            }
            Err(_) => (out, 0),
        },
        FloatFormat::E4M3 => match t.parse::<f64>() {
            Ok(v) => {
                out[0] = f64_to_e4m3(v);
                (out, 1)
            }
            Err(_) => (out, 0),
        },
        FloatFormat::E5M2 => match t.parse::<f64>() {
            Ok(v) => {
                out[0] = f64_to_e5m2(v);
                (out, 1)
            }
            Err(_) => (out, 0),
        },
    }
}

// Accept prefixes of a valid float: optional sign, digits, one dot, one 'e'/'E'
// with optional exponent sign, plus the special words inf/nan (and their prefixes).
fn is_float_prefix(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() {
        return true;
    }

    // allow "inf"/"nan"/"-inf"/"+inf" and their prefixes, case-insensitive
    let lower = t.trim_start_matches(['+', '-']).to_ascii_lowercase();
    if "inf".starts_with(&lower) || "nan".starts_with(&lower) || "infinity".starts_with(&lower) {
        return true;
    }

    let bytes = t.as_bytes();
    let mut i = 0;
    if bytes[i] == b'+' || bytes[i] == b'-' {
        i += 1;
    }
    let mut seen_dot = false;
    let mut seen_e = false;
    let mut seen_digit = false;
    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => seen_digit = true,
            b'.' if !seen_dot && !seen_e => seen_dot = true,
            b'e' | b'E' if !seen_e && seen_digit => {
                seen_e = true;
                if i + 1 < bytes.len() && (bytes[i + 1] == b'+' || bytes[i + 1] == b'-') {
                    i += 1;
                }
            }
            _ => return false,
        }
        i += 1;
    }
    true
}

// Decimal-only prefix: optional sign, digits, one dot — no scientific notation.
fn is_fixed_decimal_prefix(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() {
        return true;
    }

    let lower = t.trim_start_matches(['+', '-']).to_ascii_lowercase();
    if "inf".starts_with(&lower) || "nan".starts_with(&lower) || "infinity".starts_with(&lower) {
        return true;
    }

    let bytes = t.as_bytes();
    let mut i = 0;
    if bytes[i] == b'+' || bytes[i] == b'-' {
        i += 1;
    }
    let mut seen_dot = false;
    while i < bytes.len() {
        match bytes[i] {
            b'0'..=b'9' => {}
            b'.' if !seen_dot => seen_dot = true,
            _ => return false,
        }
        i += 1;
    }
    true
}
