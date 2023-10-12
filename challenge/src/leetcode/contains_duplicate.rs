use std::collections::HashSet;

#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut freq: HashSet<i32> = HashSet::new();
    for element in nums.iter() {
        if freq.contains(element) {
            // element is &i32 because .iter() yields references
            return true;
        }
        freq.insert(*element); // insert expects i32, so we need to dereference element
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_array() {
        let input = vec![];
        let expected = false;
        let actual = contains_duplicate(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn non_empty_array() {
        let input = vec![1, 2, 3, 4, 2];
        let expected: bool = true;
        let actual = contains_duplicate(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn all_same_array() {
        // Array of all the same element
        let input: Vec<i32> = vec![1, 1, 1, 1, 1];
        let expected: bool = true;
        let actual = contains_duplicate(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn one_element_array() {
        // Array with only one element
        let input: Vec<i32> = vec![1];
        let expected: bool = false;
        let actual = contains_duplicate(input);
        assert_eq!(expected, actual);
    }
}
