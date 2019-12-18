pub fn is_pangram(sentence: &str) -> bool {
    let s = sentence.to_lowercase();
    "abcdefghijklmnopqrstuvwxyz".chars().all(|c| s.contains(c))
}