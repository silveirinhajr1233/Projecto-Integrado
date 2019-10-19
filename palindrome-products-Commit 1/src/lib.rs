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

#[inline]
pub fn is_palindrome(n: u64) -> bool {
   //fazer
}