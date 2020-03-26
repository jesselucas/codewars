#[allow(dead_code)]
fn array_diff<T: PartialEq + Copy>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    return a.into_iter().filter(|n| !b.contains(n)).collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }
}
