pub fn compute(data: u8) -> u8 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_returns_0() {
        assert_eq!(compute(0), 0);
    }
}
