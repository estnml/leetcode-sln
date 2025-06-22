pub struct Kata;
impl Kata {
    pub fn descending_order(x: u64) -> u64 {
        let copy_x = x;


        todo!()
    }
    pub fn solve(a: usize, b: usize) -> (usize, usize) {
        let mut copy_a = a;
        let mut copy_b = b;

        if copy_a == 0 || copy_b == 0 {
            return (copy_a, copy_b);
        }

        if copy_a >= 2 * copy_b {
            copy_a = copy_a - (2 * copy_b);
            return Kata::solve(copy_a, copy_b);
        }
        if copy_b >= 2 * copy_a {
            copy_b = copy_b - (2 * copy_a);
            return Kata::solve(copy_a, copy_b);
        }

        (copy_a, copy_b)
    }
}

#[test]
fn examples_tests() {
    assert_eq!(Kata::solve(6, 19), (6, 7));
    assert_eq!(Kata::solve(2, 1), (0, 1));
    assert_eq!(Kata::solve(22, 5), (0, 1));
    assert_eq!(Kata::solve(2, 10), (2, 2));
    assert_eq!(Kata::solve(8796203, 7556), (1019, 1442));
    assert_eq!(Kata::solve(19394, 19394), (19394, 19394));
}

#[test]
fn returns_expected() {
    assert_eq!(Kata::descending_order(0), 0);
    assert_eq!(Kata::descending_order(1), 1);
    assert_eq!(Kata::descending_order(15), 51);
    assert_eq!(Kata::descending_order(1021), 2110);
    assert_eq!(Kata::descending_order(123456789), 987654321);
    assert_eq!(Kata::descending_order(145263), 654321);
    assert_eq!(Kata::descending_order(1254859723), 9875543221);
}
