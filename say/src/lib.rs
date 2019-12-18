pub fn encode(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        x => num_to_text(x),
    }
}

fn num_to_text(n: u64) -> String {
    match n {
        0 => "".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        15 => "fifteen".to_string(),
        18 => "eighteen".to_string(),
        14..=19 => format!("{}teen", num_to_text(n - 10)),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        21..=99 => format!("{}-{}", num_to_text(n - (n % 10)), num_to_text(n % 10)),
        100..=999 => format!("{} hundred {}", num_to_text(n / 100), num_to_text(n % 100)),
        1000..=999_999 => format!(
            "{} thousand {}",
            num_to_text(n / 1000),
            num_to_text(n % 1000)
        ),
        1_000_000..=999_999_999 => format!(
            "{} million {}",
            num_to_text(n / 1_000_000),
            num_to_text(n % 1_000_000)
        ),
        1_000_000_000..=999_999_999_999 => format!(
            "{} billion {}",
            num_to_text(n / 1_000_000_000),
            num_to_text(n % 1_000_000_000)
        ),
        1_000_000_000_000..=999_999_999_999_999 => format!(
            "{} trillion {}",
            num_to_text(n / 1_000_000_000_000),
            num_to_text(n % 1_000_000_000_000)
        ),
        1_000_000_000_000_000..=999_999_999_999_999_999 => format!(
            "{} quadrillion {}",
            num_to_text(n / 1_000_000_000_000_000),
            num_to_text(n % 1_000_000_000_000_000)
        ),
        _ => format!(
            "{} quintillion {}",
            num_to_text(n / 1_000_000_000_000_000_000),
            num_to_text(n % 1_000_000_000_000_000_000)
        ),
    }
    .trim()
    .to_string()
}