#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

pub fn add(a: isize, b: isize) -> isize {
    a + b
}
