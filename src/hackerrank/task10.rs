// https://www.hackerrank.com/challenges/sock-merchant/problem

pub fn sock_merchant(arr: &[i32]) -> i32 {
    let mut count = std::collections::HashMap::new();

    for &x in arr {
        *count.entry(x).or_insert(0) += 1;
    }

    let mut pairs = 0;

    for (_, v) in count {
        pairs += v / 2;
    }

    pairs
}

#[test]
fn test0() {
    let arr = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let real = sock_merchant(&arr);
    let expected = 3;

    assert_eq!(real, expected);
}
