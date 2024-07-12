pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
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

    #[test]
    fn test_gcd() {
        // Test with small positive numbers
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(24, 30), 6);
        assert_eq!(gcd(48, 18), 6);

        // Test with larger numbers
        assert_eq!(gcd(123456, 789012), 12);
        assert_eq!(gcd(987654, 321098), 2);

        // Test with identical numbers
        assert_eq!(gcd(10, 10), 10);
        assert_eq!(gcd(25, 25), 25);

        // Test with one number being a multiple of the other
        assert_eq!(gcd(12, 24), 12);
        assert_eq!(gcd(30, 15), 15);

        // Test with very large numbers
        assert_eq!(gcd(u64::MAX, u64::MAX - 1), 1);
        assert_eq!(gcd(u64::MAX - 1, u64::MAX - 2), 1);

        // Test with very small numbers
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 1), 1);
        assert_eq!(gcd(1, 2), 1);
    }

    #[test]
    #[should_panic]
    fn test_zero_input() {
        gcd(0, 1);
    }

    #[test]
    #[should_panic]
    fn test_zero_input_2() {
        gcd(1, 0);
    }

    #[test]
    #[should_panic]
    fn test_zero_input_3() {
        gcd(0, 0);
    }
}
