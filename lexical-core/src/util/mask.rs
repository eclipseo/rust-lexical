//! Bit masks for extracting bits.

use lib::mem;
use super::cast::as_cast;
use super::num::UnsignedInteger;

/// Generate a bitwise mask for the lower `n` bits.
///
/// # Examples
///
/// ```text
/// # use lexical::util::lower_n_mask;
/// # pub fn main() {
/// lower_n_mask(2u32) -> b11u32;
/// # }
/// ```
#[inline(always)]
pub(crate) fn lower_n_mask<N>(n: N)
    -> N
    where N:UnsignedInteger
{
    let bits: N = as_cast(mem::size_of::<N>() * 8);
    debug_assert!(n <= bits, "lower_n_mask() overflow in shl.");

    match n == bits {
        true  => N::max_value(),
        false => (N::ONE << n) - N::ONE,
    }
}

/// Calculate the halfway point for the lower `n` bits.
///
/// # Examples
///
/// ```text
/// # use lexical::util::lower_n_halfway;
/// # pub fn main() {
/// lower_n_halfway(2u32) -> b10u32;
/// # }
/// ```
#[inline(always)]
pub(crate) fn lower_n_halfway<N>(n: N)
    -> N
    where N:UnsignedInteger
{
    let bits: N = as_cast(mem::size_of::<N>() * 8);
    debug_assert!(n <= bits, "lower_n_halfway() overflow in shl.");

    match n.is_zero() {
        true  => N::ZERO,
        false => N::ONE << (n - N::ONE),
    }
}

/// Calculate a bitwise mask with `n` 1 bits starting at the `bit` position.
///
/// # Examples
///
/// ```text
/// # use lexical::util::internal_n_mask;
/// # pub fn main() {
/// internal_n_mask(4u32, 2u32) -> b1100u32;
/// internal_n_mask(10u32, 2u32) -> 0b1100000000;
/// # }
/// ```
#[inline(always)]
pub(crate) fn internal_n_mask<N>(bit: N, n: N)
    -> N
    where N:UnsignedInteger
{
    let bits: N = as_cast(mem::size_of::<N>() * 8);
    debug_assert!(bit <= bits, "internal_n_halfway() overflow in shl.");
    debug_assert!(n <= bits, "internal_n_halfway() overflow in shl.");
    debug_assert!(bit >= n, "internal_n_halfway() overflow in sub.");

    lower_n_mask(bit) ^ lower_n_mask(bit - n)
}


// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lower_n_mask_test() {
        assert_eq!(lower_n_mask(0u32), 0b0);
        assert_eq!(lower_n_mask(1u32), 0b1);
        assert_eq!(lower_n_mask(2u32), 0b11);
        assert_eq!(lower_n_mask(10u32), 0b1111111111);
        assert_eq!(lower_n_mask(32u32), 0b11111111111111111111111111111111);
    }

    #[test]
    fn lower_n_halfway_test() {
        assert_eq!(lower_n_halfway(0u32), 0b0);
        assert_eq!(lower_n_halfway(1u32), 0b1);
        assert_eq!(lower_n_halfway(2u32), 0b10);
        assert_eq!(lower_n_halfway(10u32), 0b1000000000);
        assert_eq!(lower_n_halfway(32u32), 0b10000000000000000000000000000000);
    }

    #[test]
    fn internal_n_mask_test() {
        assert_eq!(internal_n_mask(1u32, 0u32), 0b0);
        assert_eq!(internal_n_mask(1u32, 1u32), 0b1);
        assert_eq!(internal_n_mask(2u32, 1u32), 0b10);
        assert_eq!(internal_n_mask(4u32, 2u32), 0b1100);
        assert_eq!(internal_n_mask(10u32, 2u32), 0b1100000000);
        assert_eq!(internal_n_mask(10u32, 4u32), 0b1111000000);
        assert_eq!(internal_n_mask(32u32, 4u32), 0b11110000000000000000000000000000);
    }
}