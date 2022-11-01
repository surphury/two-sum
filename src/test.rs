#[cfg(test)]
use crate::two_sum;

#[test]
fn test_1() {
    let numbers = vec![2, 7, 11, 15];
    let target = 9;

    let result = two_sum(numbers, target);
    assert_eq!(result, vec![0, 1]);
}

#[test]
fn test_2() {
    let numbers = vec![3, 2, 4];
    let target = 6;

    let result = two_sum(numbers, target);
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_3() {
    let numbers = vec![3, 3];
    let target = 6;

    let result = two_sum(numbers, target);
    assert_eq!(result, vec![0, 1]);
}
