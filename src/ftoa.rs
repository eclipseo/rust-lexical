//! Fast lexical float-to-string conversion routines.
//!
//! The optimized routines are adapted from Andrea Samoljuk's `fpconv` library,
//! which is available [here](https://github.com/night-shift/fpconv).
//!
//! The radix-generic algorithm is adapted from the V8 codebase,
//! and may be found [here](https://github.com/v8/v8).

use sealed::mem;
use sealed::ptr;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::string::String;

#[cfg(all(feature = "alloc", not(feature = "std")))]
pub use alloc::vec::Vec;

use super::c::distance;

// MACRO

/// Fast macro absolute value calculator.
macro_rules! absv {
    ($n:expr) => (if $n < 0 { -$n } else { $n })
}

/// Fast macro minimum value calculator.
macro_rules! minv {
    ($a:expr, $b:expr) => (if $a < $b { $a } else { $b })
}

// FTOA BASE10
// -----------

// NOTATION CHAR

/// Get the exponent notation character.
///
/// After radix of base15 and higher, 'E' and 'e' are
/// part of the controlled vocabulary.
/// We use a non-standard extension of '^' to signify
/// the exponent in base15 and above.
pub extern "C" fn exponent_notation_char(base: u64)
    -> u8
{
    if base >= 15 { b'^' } else { b'e' }
}

// FP_T

/// Extended precision floating-point type.
#[repr(C)]
struct FloatType {
    frac: u64,
    exp: i32,
}

// IEEE754 CONSTANTS
// TODO(ahuszagh)   Restore...
//const EXPONENT_MASK: u64 = 0x7FF0000000000000;
//const HIDDEN_BIT: u64 = 0x0010000000000000;
//const SIGN_MASK: u64 = 0x800000000000000;
//const SIGNIFICAND_MASK: u64 = 0x000FFFFFFFFFFFFF;
//const U64_INFINITY: u64 = 0x7FF0000000000000;
//const PHYSICAL_SIGNIFICAND_SIZE: i32 = 52;
//const SIGNIFICAND_SIZE: i32 = 53;
//const EXPONENT_BIAS: i32 = 0x3FF + PHYSICAL_SIGNIFICAND_SIZE;
//const DENORMAL_EXPONENT: i32 = -EXPONENT_BIAS + 1;

// FLOATING POINT CONSTANTS
const ONE_LOG_TEN: f64 = 0.30102999566398114;
const NPOWERS: i32 = 87;
const FIRSTPOWER: i32 = -348;       // 10 ^ -348
const STEPPOWERS: i32 = 8;
const EXPMAX: i32 = -32;
const EXPMIN: i32 = -60;
const FRACMASK: u64 = 0x000FFFFFFFFFFFFF;
const EXPMASK: u64 = 0x7FF0000000000000;
const HIDDENBIT: u64 = 0x0010000000000000;
const EXPBIAS: i32 = 1023 + 52;

// BUFFER PARAMTERS
const MAX_FLOAT_SIZE: usize = 60;
const BUFFER_SIZE: usize = MAX_FLOAT_SIZE;

// LOOKUPS
const TENS: [u64; 20] = [
    10000000000000000000, 1000000000000000000, 100000000000000000,
    10000000000000000, 1000000000000000, 100000000000000,
    10000000000000, 1000000000000, 100000000000,
    10000000000, 1000000000, 100000000,
    10000000, 1000000, 100000,
    10000, 1000, 100,
    10, 1
];

const POWERS_TEN: [FloatType; 87] = [
    FloatType { frac: 18054884314459144840, exp: -1220 },
    FloatType { frac: 13451937075301367670, exp: -1193 },
    FloatType { frac: 10022474136428063862, exp: -1166 },
    FloatType { frac: 14934650266808366570, exp: -1140 },
    FloatType { frac: 11127181549972568877, exp: -1113 },
    FloatType { frac: 16580792590934885855, exp: -1087 },
    FloatType { frac: 12353653155963782858, exp: -1060 },
    FloatType { frac: 18408377700990114895, exp: -1034 },
    FloatType { frac: 13715310171984221708, exp: -1007 },
    FloatType { frac: 10218702384817765436, exp: -980 },
    FloatType { frac: 15227053142812498563, exp: -954 },
    FloatType { frac: 11345038669416679861, exp: -927 },
    FloatType { frac: 16905424996341287883, exp: -901 },
    FloatType { frac: 12595523146049147757, exp: -874 },
    FloatType { frac: 9384396036005875287, exp: -847 },
    FloatType { frac: 13983839803942852151, exp: -821 },
    FloatType { frac: 10418772551374772303, exp: -794 },
    FloatType { frac: 15525180923007089351, exp: -768 },
    FloatType { frac: 11567161174868858868, exp: -741 },
    FloatType { frac: 17236413322193710309, exp: -715 },
    FloatType { frac: 12842128665889583758, exp: -688 },
    FloatType { frac: 9568131466127621947, exp: -661 },
    FloatType { frac: 14257626930069360058, exp: -635 },
    FloatType { frac: 10622759856335341974, exp: -608 },
    FloatType { frac: 15829145694278690180, exp: -582 },
    FloatType { frac: 11793632577567316726, exp: -555 },
    FloatType { frac: 17573882009934360870, exp: -529 },
    FloatType { frac: 13093562431584567480, exp: -502 },
    FloatType { frac: 9755464219737475723, exp: -475 },
    FloatType { frac: 14536774485912137811, exp: -449 },
    FloatType { frac: 10830740992659433045, exp: -422 },
    FloatType { frac: 16139061738043178685, exp: -396 },
    FloatType { frac: 12024538023802026127, exp: -369 },
    FloatType { frac: 17917957937422433684, exp: -343 },
    FloatType { frac: 13349918974505688015, exp: -316 },
    FloatType { frac: 9946464728195732843, exp: -289 },
    FloatType { frac: 14821387422376473014, exp: -263 },
    FloatType { frac: 11042794154864902060, exp: -236 },
    FloatType { frac: 16455045573212060422, exp: -210 },
    FloatType { frac: 12259964326927110867, exp: -183 },
    FloatType { frac: 18268770466636286478, exp: -157 },
    FloatType { frac: 13611294676837538539, exp: -130 },
    FloatType { frac: 10141204801825835212, exp: -103 },
    FloatType { frac: 15111572745182864684, exp: -77 },
    FloatType { frac: 11258999068426240000, exp: -50 },
    FloatType { frac: 16777216000000000000, exp: -24 },
    FloatType { frac: 12500000000000000000, exp:  3 },
    FloatType { frac: 9313225746154785156, exp:  30 },
    FloatType { frac: 13877787807814456755, exp: 56 },
    FloatType { frac: 10339757656912845936, exp: 83 },
    FloatType { frac: 15407439555097886824, exp: 109 },
    FloatType { frac: 11479437019748901445, exp: 136 },
    FloatType { frac: 17105694144590052135, exp: 162 },
    FloatType { frac: 12744735289059618216, exp: 189 },
    FloatType { frac: 9495567745759798747, exp: 216 },
    FloatType { frac: 14149498560666738074, exp: 242 },
    FloatType { frac: 10542197943230523224, exp: 269 },
    FloatType { frac: 15709099088952724970, exp: 295 },
    FloatType { frac: 11704190886730495818, exp: 322 },
    FloatType { frac: 17440603504673385349, exp: 348 },
    FloatType { frac: 12994262207056124023, exp: 375 },
    FloatType { frac: 9681479787123295682, exp: 402 },
    FloatType { frac: 14426529090290212157, exp: 428 },
    FloatType { frac: 10748601772107342003, exp: 455 },
    FloatType { frac: 16016664761464807395, exp: 481 },
    FloatType { frac: 11933345169920330789, exp: 508 },
    FloatType { frac: 17782069995880619868, exp: 534 },
    FloatType { frac: 13248674568444952270, exp: 561 },
    FloatType { frac: 9871031767461413346, exp: 588 },
    FloatType { frac: 14708983551653345445, exp: 614 },
    FloatType { frac: 10959046745042015199, exp: 641 },
    FloatType { frac: 16330252207878254650, exp: 667 },
    FloatType { frac: 12166986024289022870, exp: 694 },
    FloatType { frac: 18130221999122236476, exp: 720 },
    FloatType { frac: 13508068024458167312, exp: 747 },
    FloatType { frac: 10064294952495520794, exp: 774 },
    FloatType { frac: 14996968138956309548, exp: 800 },
    FloatType { frac: 11173611982879273257, exp: 827 },
    FloatType { frac: 16649979327439178909, exp: 853 },
    FloatType { frac: 12405201291620119593, exp: 880 },
    FloatType { frac: 9242595204427927429, exp: 907 },
    FloatType { frac: 13772540099066387757, exp: 933 },
    FloatType { frac: 10261342003245940623, exp: 960 },
    FloatType { frac: 15290591125556738113, exp: 986 },
    FloatType { frac: 11392378155556871081, exp: 1013 },
    FloatType { frac: 16975966327722178521, exp: 1039 },
    FloatType { frac: 12648080533535911531, exp: 1066 }
];

// FPCONV GRISU

/// Find cached power of 10 from the exponent.
#[inline]
unsafe extern "C" fn find_cachedpow10(exp: i32, k: *mut i32)
    -> &'static FloatType
{
    let approx = -((exp + NPOWERS) as f64) * ONE_LOG_TEN;
    let approx = approx as i32;
    let mut idx = ((approx - FIRSTPOWER) / STEPPOWERS) as usize;

    loop {
        let current = exp + POWERS_TEN.get_unchecked(idx).exp + 64;
        if current < EXPMIN {
            idx += 1;
            continue;
        }

        if current > EXPMAX {
            idx -= 1;
            continue;
        }

        *k = FIRSTPOWER + (idx as i32) * STEPPOWERS;
        return POWERS_TEN.get_unchecked(idx);
    }
}


/// Get the double bits as u64.
#[inline(always)]
fn get_dbits(d: f64) -> u64 {
    unsafe { mem::transmute(d) }
}

/// Get the integer bits as f64.
#[inline(always)]
fn get_u64bits(i: u64) -> f64 {
    unsafe { mem::transmute(i) }
}

/// Create extended floating point type from double.
#[inline]
fn build_fp(d: f64) -> FloatType {
    // Create the extended floating point.
    let bits = get_dbits(d);
    let mut fp = FloatType {
        frac: bits & FRACMASK,
        exp: ((bits & EXPMASK) >> 52) as i32,
    };

    if fp.exp != 0 {
        fp.frac += HIDDENBIT;
        fp.exp -= EXPBIAS;
    } else {
        fp.exp = -EXPBIAS + 1;
    }

    fp
}

/// Normalize float-point number.
#[inline]
fn normalize(fp: &mut FloatType)
{
    while (fp.frac & HIDDENBIT) == 0 {
        fp.frac <<= 1;
        fp.exp -= 1;
    }

    let shift: i32 = 64 - 52 - 1;
    fp.frac <<= shift;
    fp.exp -= shift;
}

/// Get normalized boundaries for floating-point number.
fn get_normalized_boundaries(fp: &FloatType, lower: &mut FloatType, upper: &mut FloatType)
{
    upper.frac = (fp.frac << 1) + 1;
    upper.exp  = fp.exp - 1;

    while upper.frac & (HIDDENBIT << 1) == 0 {
        upper.frac <<= 1;
        upper.exp -= 1;
    }

    let u_shift: i32 = 64 - 52 - 2;

    upper.frac <<= u_shift;
    upper.exp = upper.exp - u_shift;


    let l_shift: i32 = if fp.frac == HIDDENBIT { 2 } else { 1 };

    lower.frac = (fp.frac << l_shift) - 1;
    lower.exp = fp.exp - l_shift;


    lower.frac <<= lower.exp - upper.exp;
    lower.exp = upper.exp;
}

/// Multiply two extended-precision floating point numbers.
fn multiply(a: &FloatType, b: &FloatType) -> FloatType
{
    const LOMASK: u64 = 0x00000000FFFFFFFF;

    let ah_bl = (a.frac >> 32)    * (b.frac & LOMASK);
    let al_bh = (a.frac & LOMASK) * (b.frac >> 32);
    let al_bl = (a.frac & LOMASK) * (b.frac & LOMASK);
    let ah_bh = (a.frac >> 32)    * (b.frac >> 32);

    let mut tmp = (ah_bl & LOMASK) + (al_bh & LOMASK) + (al_bl >> 32);
    // round up
    tmp += 1 << 31;

    FloatType {
        frac: ah_bh + (ah_bl >> 32) + (al_bh >> 32) + (tmp >> 32),
        exp: a.exp + b.exp + 64
    }
}

/// Round digit to sane approximation.
unsafe extern "C"
fn round_digit(digits: *mut u8, ndigits: isize, delta: u64, mut rem: u64, kappa: u64, frac: u64)
{
    while rem < frac && delta - rem >= kappa &&
           (rem + kappa < frac || frac - rem > rem + kappa - frac) {

        *digits.offset(ndigits - 1) -= 1;
        rem += kappa;
    }
}

/// Generate digits from upper and lower range on rounding of number.
unsafe extern "C"
fn generate_digits(fp: &FloatType, upper: &FloatType, lower: &FloatType, digits: *mut u8, k: *mut i32)
    -> i32
{
    let wfrac = upper.frac - fp.frac;
    let mut delta = upper.frac - lower.frac;

    let one = FloatType {
        frac: 1 << -upper.exp,
        exp: upper.exp,
    };

    let mut part1 = upper.frac >> -one.exp;
    let mut part2 = upper.frac & (one.frac - 1);

    let mut idx: isize = 0;
    let mut kappa: i32 = 10;
    // 1000000000
    let mut divp: *const u64 = TENS.as_ptr().add(10);
    while kappa > 0 {
        // Remember not to continue! This loop has an increment condition.
        let div = *divp;
        let digit = part1 / div;
        if digit != 0 || idx != 0 {
            *digits.offset(idx) = (digit as u8) + b'0';
            idx += 1;
        }

        part1 -= (digit as u64) * div;
        kappa -= 1;

        let tmp = (part1 <<-one.exp) + part2;
        if tmp <= delta {
            *k += kappa;
            round_digit(digits, idx, delta, tmp, div << -one.exp, wfrac);
            return idx as i32;
        }

        // Increment condition, DO NOT ADD continue.
        divp = divp.add(1);
    }

    /* 10 */
    let mut unit: *const u64 = TENS.as_ptr().add(18);

    loop {
        part2 *= 10;
        delta *= 10;
        kappa -= 1;

        let digit = part2 >> -one.exp;
        if digit != 0 || idx != 0 {
            *digits.offset(idx) = (digit as u8) + b'0';
            idx += 1;
        }

        part2 &= one.frac - 1;
        if part2 < delta {
            *k += kappa;
            round_digit(digits, idx, delta, part2, one.frac, wfrac * *unit);
            return idx as i32;
        }

        unit = unit.sub(1);
    }
}

/// Core Grisu2 algorithm for the float formatter.
unsafe extern "C" fn grisu2(d: f64, digits: *mut u8, k: *mut i32) -> i32
{
    let mut w = build_fp(d);

    let mut lower: FloatType = mem::uninitialized();
    let mut upper: FloatType = mem::uninitialized();
    get_normalized_boundaries(&w, &mut lower, &mut upper);

    normalize(&mut w);

    let mut ki: i32 = mem::uninitialized();
    let cp = find_cachedpow10(upper.exp, &mut ki);

    w     = multiply(&w,     &cp);
    upper = multiply(&upper, &cp);
    lower = multiply(&lower, &cp);

    lower.frac += 1;
    upper.frac -= 1;

    *k = -ki;

    return generate_digits(&w, &upper, &lower, digits, k);
}

/// Write the produced digits to string.
///
/// Adds formatting for exponents, and other types of information.
unsafe extern "C" fn emit_digits(digits: *mut u8, mut ndigits: i32, dest: *mut u8, k: i32)
    -> i32
{
    let exp = k + ndigits - 1;
    let mut exp = absv!(exp);

    // write plain integer
    if k >= 0 && exp < (ndigits + 7) {
        ptr::copy_nonoverlapping(digits, dest, ndigits as usize);
        ptr::write_bytes(dest.offset(ndigits as isize), b'0', k as usize);

        return ndigits + k;
    }

    // write decimal w/o scientific notation
    if k < 0 && (k > -7 || exp < 4) {
        let mut offset = ndigits - absv!(k);
        // fp < 1.0 -> write leading zero
        if offset <= 0 {
            offset = -offset;
            *dest = b'0';
            *dest.add(1) = b'.';
            ptr::write_bytes(dest.add(2), b'0', offset as usize);
            let dst = dest.add(offset as usize + 2);
            ptr::copy_nonoverlapping(digits, dst, ndigits as usize);

            return ndigits + 2 + offset;

        } else {
            // fp > 1.0
            ptr::copy_nonoverlapping(digits, dest, offset as usize);
            *dest.offset(offset as isize) = b'.';
            let dst = dest.offset(offset as isize + 1);
            let src = digits.offset(offset as isize);
            let count = (ndigits - offset) as usize;
            ptr::copy_nonoverlapping(src, dst, count);

            return ndigits + 1;
        }
    }

    // write decimal w/ scientific notation
    ndigits = minv!(ndigits, 18);

    let mut idx: isize = 0;
    *dest.offset(idx) = *digits;
    idx += 1;

    if ndigits > 1 {
        *dest.offset(idx) = b'.';
        idx += 1;
        let dst = dest.offset(idx);
        let src = digits.add(1);
        let count = (ndigits - 1) as usize;
        ptr::copy_nonoverlapping(src, dst, count);
        idx += (ndigits - 1) as isize;
    }

    *dest.offset(idx) = exponent_notation_char(10);
    idx += 1;

    let sign: u8 = match k + ndigits - 1 < 0 {
        true    => b'-',
        false   => b'+',
    };
    *dest.offset(idx) = sign;
    idx += 1;

    let mut cent: i32 = 0;
    if exp > 99 {
        cent = exp / 100;
        *dest.offset(idx) = (cent as u8) + b'0';
        idx += 1;
        exp -= cent * 100;
    }
    if exp > 9 {
        let dec = exp / 10;
        *dest.offset(idx) = (dec as u8) + b'0';
        idx += 1;
        exp -= dec * 10;
    } else if cent != 0 {
        *dest.offset(idx) = b'0';
        idx += 1;
    }

    let shift: u8 = (exp % 10) as u8;
    *dest.offset(idx) = shift + b'0';
    idx += 1;

    idx as i32
}

/// Filter special floating-point numbers.
unsafe extern "C" fn filter_special(fp: f64, dest: *mut u8) -> i32
{
    if fp == 0.0 {
        *dest = b'0';
        return 1;
    }

    let bits = get_dbits(fp);
    let nan = (bits & EXPMASK) == EXPMASK;

    if !nan {
        return 0;
    }

    if bits & FRACMASK != 0 {
        ptr::copy_nonoverlapping(b"NaN".as_ptr(), dest, 3);
        return 3;

    } else {
        ptr::copy_nonoverlapping(b"Infinity".as_ptr(), dest, 8);
        return 8;
    }
}

unsafe extern "C" fn fpconv_dtoa(d: f64, dest: *mut u8) -> i32
{
    let mut digits: [u8; 18] = mem::uninitialized();
    let mut str_len: i32 = 0;
    let spec = filter_special(d, dest.offset(str_len as isize));
    if spec != 0 {
        return str_len + spec;
    }

    let mut k: i32 = 0;
    let ndigits = grisu2(d, digits.as_mut_ptr(), &mut k);
    str_len += emit_digits(digits.as_mut_ptr(), ndigits, dest.offset(str_len as isize), k);

    str_len
}

unsafe extern "C" fn ftoa_base10(value: f64, first: *mut u8)
    -> *mut u8
{
    let len = fpconv_dtoa(value, first);
    first.offset(len as isize)
}

// FTOA BASEN

#[allow(unused)]
unsafe extern "C" fn ftoa_basen(value: f64, first: *mut u8, base: u64)
    -> *mut u8
{
    //ftoa_naive(value, first, last, base);
    panic!("Unruly code path!!!");
}

// TODO(ahuszagh)
//  Implement...

// FTOA

/// Check if the supplied buffer has enough range for the encoded size.
macro_rules! check_digits {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        debug_assert!(distance($first, $last) >= BUFFER_SIZE, "Need a larger buffer.");
    })
}

/// Forward the correct arguments to the implementation.
macro_rules! ftoa_forward {
    ($value:ident, $first:ident, $base:ident) => (match $base {
        10  => ftoa_base10($value, $first),
        _   => ftoa_basen($value, $first, $base),
    })
}

/// Sanitizer for an unsigned number-to-string implementation.
macro_rules! ftoa_unsafe_impl {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // Sanity checks
        debug_assert!($first <= $last);
        check_digits!($value, $first, $last, $base);

        if $value < 0.0 {
            *$first= b'-';
            $value = -$value;
            $first = $first.add(1);
        }

        ftoa_forward!($value, $first, $base)
    })
}

/// Generate the ftoa wrappers.
macro_rules! ftoa_unsafe {
    ($value:ident, $first:ident, $last:ident, $base:ident) => ({
        // check to use a temporary buffer
        let dist = distance($first, $last);
        if dist == 0 {
            // cannot write null terminator
            return $first;
        } else if dist < BUFFER_SIZE {
            // use a temporary buffer and write number to buffer
            let mut buffer: [u8; BUFFER_SIZE] = mem::uninitialized();
            let mut f = buffer.as_mut_ptr();
            let l = f.add(BUFFER_SIZE);
            let mut v = $value as f64;
            let b = $base as u64;
            ftoa_unsafe_impl!(v, f, l, b);

            // copy as many bytes as possible except the trailing null byte
            // then, write null-byte so the string is always terminated
            let length = minv!(distance(f, l), dist);
            ptr::copy_nonoverlapping(f, $first, length);
            return $first.add(length);
        } else {
            // current buffer has sufficient capacity, use it
            let mut v = $value as f64;
            let b = $base as u64;
            return ftoa_unsafe_impl!(v, $first, $last, b);
    }
    })
}

// UNSAFE API

/// Generate the unsafe wrappers.
macro_rules! unsafe_impl {
    ($func:ident, $t:ty) => (
        /// Unsafe, C-like exporter for float numbers.
        ///
        /// # Warning
        ///
        /// Do not call this function directly, unless you **know**
        /// you have a buffer of sufficient size. No size checking is
        /// done in release mode, this function is **highly** dangerous.
        /// Sufficient buffer sizes is denoted by `BUFFER_SIZE`.
        #[inline]
        pub unsafe extern "C" fn $func(
            value: $t,
            mut first: *mut u8,
            last: *mut u8,
            base: u8
        )
            -> *mut u8
        {
            ftoa_unsafe!(value, first, last, base)
        }
    )
}

unsafe_impl!(f32toa_unsafe, f32);
unsafe_impl!(f64toa_unsafe, f64);

// LOW-LEVEL API

/// Generate the low-level bytes API using wrappers around the unsafe function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! bytes_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level bytes exporter for numbers.
        pub fn $func(value: $t, base: u8)
            -> Vec<u8>
        {
            let mut buf: Vec<u8> = Vec::with_capacity(BUFFER_SIZE);
            unsafe {
                let first: *mut u8 = buf.as_mut_ptr();
                let last = first.add(buf.capacity());
                let end = $callback(value, first, last, base);
                let size = distance(first, end);
                buf.set_len(size);
            }

            buf
        }
    )
}

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(f32toa_bytes, f32, f32toa_unsafe);

#[cfg(any(feature = "std", feature = "alloc"))]
bytes_impl!(f64toa_bytes, f64, f64toa_unsafe);

/// Generate the low-level string API using wrappers around the bytes function.
#[cfg(any(feature = "std", feature = "alloc"))]
macro_rules! string_impl {
    ($func:ident, $t:ty, $callback:ident) => (
        /// Low-level string exporter for numbers.
        pub fn $func(value: $t, base: u8)
            -> String
        {
            unsafe { String::from_utf8_unchecked($callback(value, base)) }
        }
    )
}

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(f32toa_string, f32, f32toa_bytes);

#[cfg(any(feature = "std", feature = "alloc"))]
string_impl!(f64toa_string, f64, f64toa_bytes);

// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(ahuszagh)
    //  Add tests...

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn f32toa_base2_test() {
        //assert_eq!("1.001111000000110010", &f32toa_string(1.2345678901234567890e0, 2)[..20]);
    }

    #[test]
    #[cfg(any(feature = "std", feature = "alloc"))]
    fn f32toa_base10_test() {
        assert_eq!("1.234567", &f32toa_string(1.2345678901234567890e0, 10)[..8]);
    }
}
