pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    } else {
        return Some(Classification::Faltaoalgoritmo);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}