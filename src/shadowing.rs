#[cfg(test)]
mod tests {
    #[test]
    fn test_shadow() {
        let x = 5;
        assert_eq!(5, x);
        let x = x + 1;
        assert_eq!(6, x);
        { 
            let x = x * 2;
            assert_eq!(12, x);
        }
        assert_eq!(6, x);
    }
}