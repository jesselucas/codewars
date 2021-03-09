#[allow(dead_code)]
fn game(n: u64) -> Vec<u64> {
    // See if n is even
    if n % 2 == 0 {
        return vec![(n / 2) * n];
    }

    vec![n * n, 2]
}

#[allow(dead_code)]
fn testing(n: u64, exp: Vec<u64>) -> () {
    assert_eq!(game(n), exp)
}

#[test]
fn basics_game() {
    testing(4, vec![8]);
    testing(8, vec![32]);
    testing(204, vec![20808]);
    testing(807, vec![651249, 2]);
    testing(5014, vec![12570098]);
    testing(750001, vec![562501500001, 2]);
}
