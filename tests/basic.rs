use prime_iter::primes;

#[test]
fn test_functionality() {
    let primes_list: Vec<i32> = primes::<i32>().take(5).collect();
    let primes_range: Vec<u8> = primes::<u8>()
        .skip_while(|&x| x >= 10)
        .take_while(|&x| x < 100)
        .filter(|x| x % 10 + x / 10 == 11)
        .collect();

    assert_eq!(primes_list, [2, 3, 5, 7, 11]);
    assert_eq!(primes_range, [29, 47, 83]);
}

#[test]
fn test_overflow() {
    let small_primes: Vec<u8> = primes::<u8>().collect();
    let small_primes2: Vec<u8> = primes::<i32>()
        .map_while(|x| TryInto::<u8>::try_into(x).ok())
        .collect();

    assert_eq!(small_primes, small_primes2)
}
