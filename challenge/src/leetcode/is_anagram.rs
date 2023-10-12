// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.
#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .collect::<Vec<_>>();
    let t = t
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .collect::<Vec<_>>();

    if s.len() != t.len() {
        return false;
    }

    let mut freq_s = [0; 26];
    let mut freq_t = [0; 26];

    for char in s {
        let index = (char as u8 - b'a' as u8) as usize;
        freq_s[index] += 1;
    }

    for char in t {
        let index = (char as u8 - b'a' as u8) as usize;
        freq_t[index] += 1;
    }

    freq_s == freq_t
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_one() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        let expected = true;
        let actual = is_anagram(s, t);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(is_anagram(String::from(""), String::from("")), true);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(is_anagram(String::from("a"), String::from("ab")), false);
        assert_eq!(is_anagram(String::from("ab"), String::from("a")), false);
    }

    #[test]
    fn test_same_characters_different_frequencies() {
        assert_eq!(is_anagram(String::from("aa"), String::from("a")), false);
        assert_eq!(is_anagram(String::from("aabb"), String::from("abab")), true);
    }

    #[test]
    fn test_strings_with_spaces_or_other_characters() {
        assert_eq!(
            is_anagram(String::from("listen"), String::from("silent")),
            true
        );
        assert_eq!(
            is_anagram(String::from("hello world"), String::from("worldhello ")),
            true
        );
        assert_eq!(
            is_anagram(String::from("hello world"), String::from("world hello")),
            true
        );
    }

    // Assuming the function is case-sensitive
    #[test]
    fn test_uppercase_vs_lowercase() {
        assert_eq!(
            is_anagram(String::from("Listen"), String::from("Silent")),
            false
        );
    }

    #[test]
    fn test_strings_with_numbers_or_symbols() {
        assert_eq!(is_anagram(String::from("123"), String::from("321")), true);
        assert_eq!(is_anagram(String::from("!@#"), String::from("#@!")), true);
    }

    #[test]
    fn test_strings_with_repeated_characters() {
        assert_eq!(is_anagram(String::from("aaa"), String::from("aaa")), true);
        assert_eq!(is_anagram(String::from("aaa"), String::from("aab")), false);
    }
}
