#[allow(dead_code)]
fn count_sheep(sheep: &[bool]) -> u8 {
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#examples-10
    // Filter closure is a double reference
    sheep.iter().filter(|&s| *s).count() as u8
}

#[test]
fn returns_correct_sheep_count() {
    assert_eq!(count_sheep(&[false]), 0);
    assert_eq!(count_sheep(&[true]), 1);
    assert_eq!(count_sheep(&[true, false]), 1);
}
