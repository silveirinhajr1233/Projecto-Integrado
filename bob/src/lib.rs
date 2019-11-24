pub fn reply(message: &str) -> &str {
    let cleaned_message = message
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .collect::<Vec<char>>();
    let is_uppercase = (!message.is_empty() && cleaned_message.iter().count() > 0)
        && cleaned_message.iter().all(|x| x.is_uppercase());
    match message.trim() {
        m if m.ends_with('?') => {
            if is_uppercase {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        _ if is_uppercase => "Whoa, chill out!",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}