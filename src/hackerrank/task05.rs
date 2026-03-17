// https://www.hackerrank.com/challenges/apple-and-orange/problem

pub fn count_apples_and_oranges(
    s: i32,
    t: i32,
    a: i32,
    b: i32,
    apples: &[i32],
    oranges: &[i32],
) -> (i32, i32) {
    let mut apple_count = 0;
    let mut orange_count = 0;

    for &x in apples {
        let pos = a + x;
        if pos >= s && pos <= t {
            apple_count += 1;
        }
    }

    for &x in oranges {
        let pos = b + x;
        if pos >= s && pos <= t {
            orange_count += 1;
        }
    }

    (apple_count, orange_count)
}

#[test]
fn test0() {
    let s = 7;
    let t = 11;
    let a = 5;
    let b = 15;

    let apples = vec![-2, 2, 1];
    let oranges = vec![5, -6];

    let real = count_apples_and_oranges(s, t, a, b, &apples, &oranges);

    let expected = (1, 1);

    assert_eq!(real, expected);
}
