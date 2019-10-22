/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);
    if s1.len() != s2.len() {
        return None;
    }

    Some(s1.chars().zip(s1.chars()).filter(|&(s1, s2)| s1 != s2).count())
}
