pub fn generate_regex(sequence: &Vec<u32>) -> String {
    let mut regex = String::new();
    regex.push_str(r"^\.*");

    let mut first = true;
    for number in sequence {
        if !first {
            regex.push_str(r"\.+");
        } else {
            first = false;
        }
        regex.push_str(&format!(r"#{{{}}}", number));
    }

    regex.push_str(r"\.*$");

    regex
}
