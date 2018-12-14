//! Precalculated large powers for prime numbers for `b^2^i`.
//!
//! We only need powers such that `b^n <= 2^1075` for `bigcomp`.
//! However, for `algorithm_m`, we need at least as many digits as are
//! input. We tentatively accept up to ~2^15.
//!
//! The larger powers are **quite** large (~3Kb per base), so we'd rather
//! not include them in binaries unless necessary.

use super::math::Limb;

#[cfg(target_pointer_width = "16")]
use super::large_powers_16::*;

#[cfg(target_pointer_width = "32")]
use super::large_powers_32::*;

#[cfg(target_pointer_width = "64")]
use super::large_powers_64::*;

// HELPER

/// Get the correct large power from the base.
#[allow(dead_code)]
pub(in atof::algorithm) fn get_large_powers(base: u32)
    -> &'static [&'static [Limb]]
{
    match base {
        3  => &POW3,
        5  => &POW5,
        7  => &POW7,
        11  => &POW11,
        13  => &POW13,
        17  => &POW17,
        19  => &POW19,
        23  => &POW23,
        29  => &POW29,
        31  => &POW31,
        _  => unreachable!(),
    }
}
