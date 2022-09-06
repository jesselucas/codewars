#[allow(dead_code)]
fn zeros_safety(n: u64) -> u64 {
    let mut r = 0;
    let mut n = n / 5;
    while n > 0 {
        r += n;
        n /= 5;
    }
    r
}

#[allow(dead_code)]
fn zeros(n: u64) -> u64 {
    let max = n.log(5) as f32.floor();

	0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        assert_eq!(zeros(0), 0);
        assert_eq!(zeros(6), 1);
        assert_eq!(zeros(14), 2);
        assert_eq!(zeros(30), 7);
        assert_eq!(zeros(1000), 249);
        assert_eq!(zeros(100000), 24999);
        assert_eq!(zeros(1000000000), 249999998);
    }
}
