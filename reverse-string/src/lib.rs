pub fn reverse(input: &str) -> String {

	let mut s = String::from("");

	for i in (input.chars()).rev(){

		s.push(i);
	}
    return s;
}

