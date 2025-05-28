//! # lens_on
//!
//! A library for querying Uniswap V3 Ticks using lens_on onchain contract

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![warn(
    missing_copy_implementations,
    missing_debug_implementations,
    unreachable_pub,
    clippy::missing_const_for_fn,
    clippy::missing_inline_in_public_items,
    clippy::redundant_clone,
    rustdoc::all
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate alloc;

pub mod uni_obrv;

#[cfg(test)]
mod tests;


