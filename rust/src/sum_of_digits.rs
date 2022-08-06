#[allow(dead_code)]
// Base 10 radix, radix - 1 = 9
fn digital_root(n: i64) -> i64 {
    (n - 1) % 9 + 1
}

#[allow(dead_code)]
fn digital_root_orig(n: i64) -> i64 {
    let d = digits(n);
    let mut sum = 0;
    let mut place = 1;
    if d > 1 {
        for _ in 0..d {
            sum += place_val(n, place);
            place *= 10;
        }
    }

    if digits(sum) > 1 {
        println!("here sum? {}", sum);
        return digital_root(sum);
    }

    sum
}

fn digits(n: i64) -> i64 {
    let mut length = 0;
    let mut place = 1;
    while place <= n {
        length += 1;
        place *= 10;
    }

    length
}

fn place_val(n: i64, place: i64) -> i64 {
    ((n / place) % 10).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(digital_root(16), 7);
        assert_eq!(digital_root(942), 6);
        assert_eq!(digital_root(132189), 6);
        assert_eq!(digital_root(493193), 2);
    }
}
