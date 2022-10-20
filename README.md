# prime-iter
A somewhat optimized incremental-sieve based prime generator.

## Examples
The interface is given in the form of an iterator, so usage is very simple and idiomatic:
```rust
let fifty_second_prime = prime_iter::primes::<i32>().nth(51).unwrap();

assert_eq!(fifty_second_prime, 239);
```
```rust
let prime_sum: i32 = prime_iter::primes::<i32>().take(100).sum();

assert_eq!(prime_sum, 24133);
```
```rust
let two_digit_primes: Vec<i32> = prime_iter::primes::<i32>().skip_while(|&x| x < 10).take_while(|&x| x < 100).collect();

assert_eq!(two_digit_primes, [11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
```
And of course `for` loops work too:
```rust
for prime in prime_iter::primes::<i32>() {
    if prime % 10 == 1 {
        println!("{prime}");
    }
}
```

## `no_std` Support
`prime-iter` supports no-std environments, however it does use allocations. Disable the `std` feature,
which is enabled by default, for a no_std environment.

## Installation
Either add this line to your `Cargo.toml`:
```toml
prime-iter = "0.1"
```
Or simply run `cargo add prime-iter`.

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.