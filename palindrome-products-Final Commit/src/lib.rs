#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    pub factors: Vec<(u64, u64)>,
}

impl Palindrome {
    /// Criação do palindromo com os seguintes factores iniciais
    pub fn new(mut a: u64, mut b: u64) -> Palindrome {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        Palindrome {
            factors: vec![(a, b)],
        }
    }

    /// Return do valor do Palindromo
    pub fn value(&self) -> u64 {
        self.factors[0].0 * self.factors[0].1
    }

    /// por novos factores num palindromo ja existente
    pub fn insert(&mut self, mut a: u64, mut b: u64) {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        self.factors.push((a, b));
        self.factors.sort_unstable();
        self.factors.dedup();
    }
}

/// return do palíndromo composto pelos produtos (min, max)
pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result = None;

    for a in min..=max {
        for b in min..=a {
            if is_palindrome(a * b) {
                result = match result {
                    None => Some((Palindrome::new(a, b), Palindrome::new(a, b))),
                    Some((mut minp, mut maxp)) => {
                        if a * b < minp.value() {
                            minp = Palindrome::new(a, b);
                        } else if a * b == minp.value() {
                            minp.insert(a, b);
                        }
                        if a * b > maxp.value() {
                            maxp = Palindrome::new(a, b);
                        } else if a * b == maxp.value() {
                            maxp.insert(a, b);
                        }
                        Some((minp, maxp))
                    }
                };
            }
        }
    }

    result
}

#[inline]
pub fn is_palindrome(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    s.iter().zip(s.iter().rev()).all(|(a, b)| a == b)
}