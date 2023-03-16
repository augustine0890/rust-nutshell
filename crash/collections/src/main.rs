use std::collections::HashMap;

fn main() {
    let poem = vec![
        "I do not like them in a box",
        "I do not like them with a fox",
        "I do not like them in a house",
        "I do not like them with a mouse",
        "I do not like them here or there",
        "I do not like them anywhere",
        "I do not like green eggs and ham",
    ];

    let mut word_map: HashMap<&str, u32> = HashMap::new();
    for line in &poem {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in &words {
            *word_map.entry(word).or_insert(0) += 1;
        }
    }

    let mut sorted_pairs: Vec<_> = word_map.iter().collect();
    sorted_pairs.sort_by(|a, b| b.1.cmp(&a.1)); // descending
    // sorted_pairs.sort_by(|a, b| a.1.cmp(&b.1)); // ascending

    for (key, value) in sorted_pairs {
        println!("{} {}", key, value);
    }
}
