#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}

pub fn add(a: i64, b: i64) -> i64 {
    a + b
}
