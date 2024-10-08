pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

pub fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_multiply_ambiguous() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(2, 0), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn test_multiply_zero() {
        assert_eq!(multiply(2, 0), 0);
    }
}
