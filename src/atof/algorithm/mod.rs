//! Algorithms for parsing strings to floats.

// Hide implementation details.
mod exponent;

#[cfg(any(test, feature = "algorithm_m"))]
pub(crate) mod algorithm_m;

cfg_if! {
if #[cfg(any(test, not(feature = "imprecise")))] {
// Needed for the actual items.
pub(crate) mod bigcomp;
mod cached;
mod cached_float80;
mod cached_float160;
mod large_powers;
mod math;
mod small_powers;

#[cfg(target_pointer_width = "16")]
mod large_powers_16;

#[cfg(target_pointer_width = "16")]
mod small_powers_16;

#[cfg(target_pointer_width = "32")]
mod large_powers_32;

#[cfg(target_pointer_width = "32")]
mod small_powers_32;

#[cfg(target_pointer_width = "64")]
mod large_powers_64;

#[cfg(target_pointer_width = "64")]
mod small_powers_64;

}}  // cfg_if

// Export algorithms.
#[cfg(any(test, not(feature = "imprecise")))]
pub(crate) mod precise;

#[cfg(any(test, feature = "imprecise"))]
pub(crate) mod imprecise;
