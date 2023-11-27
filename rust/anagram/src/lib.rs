use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn sort_word(word: &str) -> String {
    let lowercase_word = word.to_lowercase();
    let mut word_scalar_values: Vec<_> = lowercase_word.graphemes(true).collect();
    word_scalar_values.sort();
	word_scalar_values.join("")
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    let word_sorted_string = sort_word(word);
    for each_word in possible_anagrams {
        if each_word.len() != word.len() { continue; }
        if each_word.to_lowercase().to_string() == word.to_lowercase().to_string() { continue; }

        let each_word_sorted_string = sort_word(each_word);
        if each_word_sorted_string != word_sorted_string {
            continue;
        } else {
            result.insert(each_word);
        }
    }
    result
}
