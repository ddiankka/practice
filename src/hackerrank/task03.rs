// https://www.hackerrank.com/challenges/staircase/problem

pub fn staircase(n: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        result.push(format!("{}{}", spaces, hashes));
    }

    result
}

#[test]
fn test0() {
    let real = staircase(4);

    let expected = vec![
        "   #".to_string(),
        "  ##".to_string(),
        " ###".to_string(),
        "####".to_string(),
    ];

    assert_eq!(real, expected);
}
