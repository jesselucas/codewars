#[allow(dead_code)]

/**
 * i increments the value (initially 0)
 * d decrements the value
 * s squares the value
 * o outputs the value into the return array
**/
fn parse(code: &str) -> Vec<i32> {
    let mut n: i32 = 0;
    let mut o = Vec::new();
    for c in code.chars() {
        match c {
            'i' => n += 1,
            'd' => n -= 1,
            's' => n *= n,
            'o' => o.push(n),
            _ => println!("invalid code"),
        }
    }

    o
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}
