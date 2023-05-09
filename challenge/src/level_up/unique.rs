#![allow(dead_code)]
use std::collections::HashSet;

// fn unique(mut a: Vec<i32>) -> Vec<i32> {
// a.sort();
// a.dedup();
// a
// }

// fn unique<T: Ord>(mut a: Vec<T>) -> Vec<T> {
// a.sort();
// a.dedup();
// a
// }

fn unique(a: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut unique = a
        .into_iter()
        .filter(|x| seen.insert(*x))
        .collect::<Vec<i32>>();
    unique.sort();
    unique
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        let input = vec![2, 1, 1];
        let answer = unique(input);
        println!("unique items {:?}", answer);
    }

    // #[test]
    // fn empty_list() {
    // let input: Vec<i64> = vec![];
    // let expected_output: Vec<_> = vec![];
    // let actual_output: Vec<_> = unique(input);
    // assert_eq!(actual_output, expected_output);
    // }

    #[test]
    fn empty_list() {
        let input = vec![];
        let expected_output = vec![];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1, 4, 5];
        let expected_output = vec![1, 4, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1, 5, 2];
        let expected_output = vec![1, 2, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list_with_duplicates() {
        let input = vec![1, 5, 2, 2, 1];
        let expected_output = vec![1, 2, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn sorted_list_with_duplicates() {
        let mut input = vec![1, 5, 2, 2, 1];
        input.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let expected_output = vec![1, 2, 5];
        let actual_output = unique(input);
        assert_eq!(actual_output, expected_output);
    }
}
