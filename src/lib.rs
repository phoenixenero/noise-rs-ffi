// Copyright 2016 Phoenix Enero.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Exposes a subset of functions from `noise-rs` in a C-compatible FFI
//! interface.
//!
//! ## Usage
//!
//! Use this library in a non-Rust setting. If you're programming in rust, just
//! use `noise-rs` directly.
//!
//! ### Creating a Seed object
//!
//! Create a Seed using the `noise_seed_new` function. It accepts an unsigned
//! 32-bit integer and returns a new pointer to the Seed object.
//!
//! ```c
//! Seed *seed = noise_seed_new((uint32_t) 42);
//! ```
//!
//! ### Using the Seed object for generating noise
//!
//! You can now use that pointer with the noise generation functions. Here's an
//! example using 3D Perlin noise.
//!
//! ```c
//! double val = noise_perlin3(seed, 0.2, 0.3, 1.5);
//! ```
//!
//! ### Freeing the Seed object
//!
//! When you're done using the Seed object, you'll need to free it. To do that,
//! use the `noise_seed_delete` function.
//!
//! ```c
//! noise_seed_delete(seed);
//! ```

extern crate libc;
extern crate noise;

use libc::{c_double, uint32_t};

/// Moves `x` onto the heap and returns a mutable pointer to it.
fn heap_mut_ptr<T>(x: T) -> *mut T {
    Box::into_raw(Box::new(x))
}

/// Returns a pointer to a heap-allocated `noise::Seed`. You must call
/// `noise_seed_delete` to free the returned seed.
#[no_mangle]
pub unsafe extern "C" fn noise_seed_new(seed: uint32_t) -> *mut noise::Seed {
    heap_mut_ptr(noise::Seed::new(seed as u32))
}

/// Delete the seed object.
///
/// ## C function signature
#[no_mangle]
pub unsafe extern "C" fn noise_seed_delete(seed: *mut noise::Seed) {
    let seed = Box::from_raw(seed);
    drop(seed)
}

macro_rules! wrap_noise {
    ($wrapper_name:ident, $t:ty, $alg:expr, 2) => {
        wrap_noise!($wrapper_name, $t, $alg, x, y);
    };
    ($wrapper_name:ident, $t:ty, $alg:expr, 3) => {
        wrap_noise!($wrapper_name, $t, $alg, x, y, z);
    };
    ($wrapper_name:ident, $t:ty, $alg:expr, 4) => {
        wrap_noise!($wrapper_name, $t, $alg, x, y, z, w);
    };
    ($wrapper_name:ident, $t:ty, $alg:expr $(, $arg:ident)*) => {
        /// Generate noise values at position using a Seed object.
        #[no_mangle]
        pub unsafe fn $wrapper_name(seed: *mut noise::Seed $(, $arg: $t)*) -> $t {
            let seed: &noise::Seed = &*seed;
            $alg(&seed, &[$($arg),*])
        }
    };
}

// Perlin Noise
wrap_noise!(noise_perlin2, c_double, noise::perlin2, 2);
wrap_noise!(noise_perlin3, c_double, noise::perlin3, 3);
wrap_noise!(noise_perlin4, c_double, noise::perlin4, 4);

// OpenSimplex Noise
wrap_noise!(noise_open_simplex2, c_double, noise::open_simplex2, 2);
wrap_noise!(noise_open_simplex3, c_double, noise::open_simplex3, 3);

// Cell noise (euclidean distance)
wrap_noise!(noise_cell2_value, c_double, noise::cell2_value, 2);
wrap_noise!(noise_cell3_value, c_double, noise::cell3_value, 3);
wrap_noise!(noise_cell4_value, c_double, noise::cell4_value, 4);

wrap_noise!(noise_cell2_range, c_double, noise::cell2_range, 2);
wrap_noise!(noise_cell3_range, c_double, noise::cell3_range, 3);
wrap_noise!(noise_cell4_range, c_double, noise::cell4_range, 4);

wrap_noise!(noise_cell2_range_inv, c_double, noise::cell2_range_inv, 2);
wrap_noise!(noise_cell3_range_inv, c_double, noise::cell3_range_inv, 3);
wrap_noise!(noise_cell4_range_inv, c_double, noise::cell4_range_inv, 4);

// Cell noise (Manhattan distance)
wrap_noise!(noise_cell2_manhattan_value, c_double, noise::cell2_manhattan_value, 2);
wrap_noise!(noise_cell3_manhattan_value, c_double, noise::cell3_manhattan_value, 3);
wrap_noise!(noise_cell4_manhattan_value, c_double, noise::cell4_manhattan_value, 4);

wrap_noise!(noise_cell2_manhattan_range, c_double, noise::cell2_manhattan, 2);
wrap_noise!(noise_cell3_manhattan_range, c_double, noise::cell3_manhattan, 3);
wrap_noise!(noise_cell4_manhattan_range, c_double, noise::cell4_manhattan, 4);

wrap_noise!(noise_cell2_manhattan_range_inv, c_double, noise::cell2_manhattan_inv, 2);
wrap_noise!(noise_cell3_manhattan_range_inv, c_double, noise::cell3_manhattan_inv, 3);
wrap_noise!(noise_cell4_manhattan_range_inv, c_double, noise::cell4_manhattan_inv, 4);
