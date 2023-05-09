#![allow(dead_code)]
pub fn median(mut a: Vec<f32>) -> Option<f32> {
    if a.is_empty() {
        return None;
    }

    a.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let n_elements = a.len();
    let middle = n_elements / 2;
    let med = if n_elements % 2 == 0 {
        (a[middle - 1] + a[middle]) / 2.0
    } else {
        a[middle]
    };

    Some(med)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_list() {
        let input = vec![];
        let expected_output = None;
        let actutal_output = median(input);
        assert_eq!(actutal_output, expected_output);
    }

    #[test]
    fn sorted_list() {
        let input = vec![1.0, 4.0, 5.0];
        let expected_output = Some(4.0);
        let actutal_output = median(input);
        assert_eq!(actutal_output, expected_output);
    }

    #[test]
    fn even_length() {
        let input = vec![1.0, 3.0, 5.0, 6.0];
        let expected_output = Some(4.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn unsorted_list() {
        let input = vec![1.0, 5.0, 2.0];
        let expected_output = Some(2.0);
        let actual_output = median(input);
        assert_eq!(actual_output, expected_output);
    }
}
