#[allow(dead_code)]
fn solution(phrase: &str) -> String {
    return phrase.chars().rev().collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_test() {
        solution("world");
        assert_eq!(solution("world"), "dlrow");
    }
}
