use crate::regex::generate_regex;

#[derive(Debug, Clone)]
pub struct Row {
    pub data: String,
    pub sequence: Vec<u32>,

    regex: String,
}

impl Row {
    pub fn new(data: String, sequence: Vec<u32>) -> Self {
        let regex = generate_regex(&sequence);
        Row { data, sequence, regex }
    }

    pub fn is_valid(&self, permutation: &str) -> bool {
        let re = regex::Regex::new(&self.regex).unwrap();

        re.is_match(permutation)
    }
}
