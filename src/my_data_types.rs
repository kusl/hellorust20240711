#[cfg(test)]
mod tests {
    #[test]
    fn test_not_a_number() {
        let guess: u32 = "42".parse().expect("Avocado toast!");
        assert_eq!(42, guess);
    }
}