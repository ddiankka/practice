// https://www.hackerrank.com/challenges/between-two-sets/problem

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut count = 0;

    for x in 1..=100 {
        let mut ok = true;

        for &ai in a {
            if x % ai != 0 {
                ok = false;
            }
        }

        for &bi in b {
            if bi % x != 0 {
                ok = false;
            }
        }

        if ok {
            count += 1;
        }
    }

    count
}

#[test]
fn test0() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];

    let real = get_total_x(&a, &b);
    let expected = 3;

    assert_eq!(real, expected);
}
