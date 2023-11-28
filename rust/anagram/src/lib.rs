use std::collections::HashSet;
use unicode_segmentation::UnicodeSegmentation;

fn sort_by_graphmes(word: &str) -> String {
    let lowercase_word = word.to_lowercase();
    let mut graphemes = lowercase_word
        .graphemes(true).collect::<Vec<&str>>();
    graphemes.sort();
	graphemes.join("")
}

fn is_an_anagram_of(candidate: &str, target: &str) -> bool {
    if candidate.to_lowercase().to_string() == target.to_lowercase().to_string() ||
       sort_by_graphmes(candidate) != sort_by_graphmes(target) {
        false
    } else {
        true
    }
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();
    for possible_anagram in possible_anagrams {
        if is_an_anagram_of(possible_anagram, word) {
            result.insert(possible_anagram);
        }
    }
    result
}
