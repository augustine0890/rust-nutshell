pub fn median(a: Vec<f32>) -> Option<f32> {
    let len = a.len();
    if len == 0 {
        None
    } else {
        let mut sorted_a = a.clone();
        sorted_a.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = len / 2;
        if mid % 2 == 0 {
            Some((sorted_a[mid - 1] + sorted_a[mid]) / 2.0)
        } else {
            Some(sorted_a[mid])
        }
    }
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actutal_output = median(input);
    assert_eq!(actutal_output, expected_output);
}
