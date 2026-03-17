// https://www.hackerrank.com/challenges/kangaroo/problem

pub fn kangaroo(x1: i32, v1: i32, x2: i32, v2: i32) -> bool {
    if v1 <= v2 {
        return false;
    }

    let diff_pos = x2 - x1;
    let diff_vel = v1 - v2;

    diff_pos % diff_vel == 0
}

#[test]
fn test0() {
    let real = kangaroo(0, 3, 4, 2);
    let expected = true;
    assert_eq!(real, expected);
}
