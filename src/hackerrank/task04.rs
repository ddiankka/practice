// https://www.hackerrank.com/challenges/grading/problem

pub fn grading_students(grades: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for &g in grades {
        if g < 38 {
            result.push(g);
        } else {
            let next = ((g / 5) + 1) * 5;
            if next - g < 3 {
                result.push(next);
            } else {
                result.push(g);
            }
        }
    }

    result
}

#[test]
fn test0() {
    let grades = vec![73, 67, 38, 33];

    let real = grading_students(&grades);

    let expected = vec![75, 67, 40, 33];

    assert_eq!(real, expected);
}
