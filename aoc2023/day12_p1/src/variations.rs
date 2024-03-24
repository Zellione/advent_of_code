pub fn generate_variations(length: usize, variations: usize, chars: Vec<char>) -> Vec<String> {
    let mut possibilities: Vec<String> = Vec::new();

    while possibilities.len() < variations {
        let mut possibility = String::new();
        let mut variation = possibilities.len();

        for _ in 0..length {
            possibility.push(chars[variation % chars.len()]);
            variation /= chars.len();
        }

        possibilities.push(possibility);
    }

    possibilities
}
