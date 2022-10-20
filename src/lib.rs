#![cfg_attr(not(feature = "std"), no_std)]
//! A somewhat optimized incremental-sieve based prime generator.
//!
//! ## Examples
//! The interface is given in the form of an iterator, so usage is very simple and idiomatic:
//! ```
//! let fifty_second_prime = prime_iter::primes::<i32>().nth(51).unwrap();
//!
//! assert_eq!(fifty_second_prime, 239);
//! ```
//! ```
//! let prime_sum: i32 = prime_iter::primes::<i32>().take(100).sum();
//!
//! assert_eq!(prime_sum, 24133);
//! ```
//! ```
//! let two_digit_primes: Vec<i32> = prime_iter::primes::<i32>().skip_while(|&x| x < 10).take_while(|&x| x < 100).collect();
//!
//! assert_eq!(two_digit_primes, [11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
//! ```
//! And of course `for` loops work too:
//! ```no_run
//! for prime in prime_iter::primes::<i32>() {
//!     if prime % 10 == 1 {
//!         println!("{prime}");
//!     }
//! }
//! ```
//!
//! ## `no_std` Support
//! `prime-iter` supports no-std environments, however it does use allocations. Disable the `std` feature,
//! which is enabled by default, for a no_std environment.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// Returns an iterator that provides all of the primes that fit into `T`, in increasing order.
///
/// `T` may only be a primitive integer type (`iXX`, `uXX` and `isize`, `usize`)
pub fn primes<T>() -> Primes<T>
where
    Primes<T>: Default + Iterator<Item = T>,
{
    Primes::default()
}

/// The iterator type returned by [primes].
pub struct Primes<T> {
    primes: Vec<T>,
    multiples: Vec<T>,
    overflowed: Vec<bool>,
    current: T,
    done: bool,
}

macro_rules! impl_primes {
    ($ty:path) => {
        impl Default for Primes<$ty> {
            fn default() -> Self {
                Self {
                    primes: Default::default(),
                    multiples: Default::default(),
                    overflowed: Default::default(),
                    current: 2,
                    done: false,
                }
            }
        }

        impl Iterator for Primes<$ty> {
            type Item = $ty;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    return None;
                }
                if self.current == 2 {
                    self.current += 1;
                    return Some(2);
                }

                loop {
                    let mut is_prime = true;

                    'primeloop: for (i, &prime) in self.primes.iter().enumerate() {
                        if self.overflowed[i] {
                            continue;
                        }
                        while self.multiples[i] < self.current {
                            match self.multiples[i].checked_add(prime) {
                                Some(x) => self.multiples[i] = x,
                                None => {
                                    self.overflowed[i] = true;
                                    continue 'primeloop;
                                }
                            }
                        }

                        if self.multiples[i] == self.current {
                            is_prime = false;
                            break;
                        }
                    }

                    let current = self.current;

                    if is_prime {
                        if let Some(res) = self.current.checked_mul(self.current) {
                            self.primes.push(self.current);
                            self.multiples.push(res);
                            self.overflowed.push(false);
                        }
                    }

                    match self.current.checked_add(2) {
                        Some(num) => self.current = num,
                        None => {
                            self.done = true;
                            break None;
                        }
                    }

                    if is_prime {
                        break Some(current);
                    }
                }
            }
        }
    };

    ($($ty:path),+) => {
        $(
            impl_primes!($ty);
        )+
    };
}

impl_primes!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
