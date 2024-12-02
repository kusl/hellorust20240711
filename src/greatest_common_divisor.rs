pub fn gcd(mut n: u32, mut m: u32) -> u32 {
    if n == 0 || m == 0 {
        return std::cmp::max(n, m);
    }
    while m != 0 {
        let t = m;
        m = n % m;
        n = t;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_gcd() {
        // Test with small positive numbers
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(24, 30), 6);
        assert_eq!(gcd(48, 18), 6);

        // Test with larger numbers
        assert_eq!(gcd(123_456, 789_012), 12);
        assert_eq!(gcd(987_654, 321_098), 2);

        // Test with identical numbers
        assert_eq!(gcd(10, 10), 10);
        assert_eq!(gcd(25, 25), 25);

        // Test with one number being a multiple of the other
        assert_eq!(gcd(12, 24), 12);
        assert_eq!(gcd(30, 15), 15);

        // Test with very large numbers
        assert_eq!(gcd(u32::MAX, u32::MAX - 1), 1);
        assert_eq!(gcd(u32::MAX - 1, u32::MAX - 2), 1);

        // Test with very small numbers
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(1, 2), 1);

        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                       3 * 7 * 11 * 13 * 19),
                   3 * 11);
    }

    #[test]
    fn test_zero_input() {
        assert_eq!(gcd(0, 1), 1);
        assert_eq!(gcd(1, 0), 1);
    }

    #[test]
    fn test_zero_input_3() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_prime_numbers() {
        assert_eq!(gcd(13, 27), 1);
        assert_eq!(gcd(17, 19), 1);
        assert_eq!(gcd(29, 58), 29);
    }

    #[test]
    fn test_mixed_numbers() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn test_large_prime_numbers() {
        assert_eq!(gcd(1_000_003, 1_000_019), 1);
    }

    #[test]
    fn test_large_composite_numbers() {
        assert_eq!(gcd(1_000_000, 500_000), 500_000);
    }

    #[test]
    fn test_one_is_zero() {
        assert_eq!(gcd(0, 5), 5);
        assert_eq!(gcd(5, 0), 5);
    }

    #[test]
    fn test_both_are_zero() {
        assert_eq!(gcd(0, 0), 0);
    }

    #[test]
    fn test_one_is_one() {
        assert_eq!(gcd(1, 5), 1);
        assert_eq!(gcd(5, 1), 1);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(gcd(1_000_000_000, 500_000_000), 500_000_000);
    }

    #[test]
    fn test_large_numbers_performance() {
        assert_eq!(gcd(1_000_000_000, 500_000_000), 500_000_000);
        assert_eq!(gcd(1_000_000_000, 1), 1);
    }

    #[test]
    fn test_large_numbers_performance_single() {
        let start = Instant::now();
        assert_eq!(gcd(1_000_000_000, 500_000_000), 500_000_000);
        let duration = start.elapsed();
        println!("Time taken for gcd(1_000_000_000, 500_000_000): {:?}", duration);

        let start = Instant::now();
        assert_eq!(gcd(1_000_000_000, 1), 1);
        let duration = start.elapsed();
        println!("Time taken for gcd(1_000_000_000, 1): {:?}", duration);
    }

    #[test]
    fn test_large_numbers_performance_multiple() {
        let iterations = 1_000;
        let mut total_duration = 0;

        for _ in 0..iterations {
            let start = Instant::now();
            let result = gcd(1_000_000_000, 500_000_000);
            assert_eq!(result, 500_000_000);
            total_duration += start.elapsed().as_nanos();
        }
        println!("Average time taken for gcd(1_000_000_000, 500_000_000): {} ns", total_duration / iterations);

        total_duration = 0;
        for _ in 0..iterations {
            let start = Instant::now();
            let result = gcd(1_000_000_000, 1);
            assert_eq!(result, 1);
            total_duration += start.elapsed().as_nanos();
        }
        println!("Average time taken for gcd(1_000_000_000, 1): {} ns", total_duration / iterations);
    }
}
