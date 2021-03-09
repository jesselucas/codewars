#[allow(dead_code)]
fn digitize(n: u64) -> Vec<u8> {
    let mut nums: u64 = n; // copy to mutate
    let mut v = Vec::new();
    loop {
        // Base 10 modulo to get last digit
        let m = nums % 10;
        v.push(m as u8);
        nums = nums / 10; // remove last digit

        if nums <= 0 {
            break;
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
        assert_eq!(digitize(0), vec![0]);
    }
}
