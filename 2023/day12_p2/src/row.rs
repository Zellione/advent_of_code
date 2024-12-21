#[derive(Debug, Clone)]
pub struct Row {
    pub data: String,
    pub sequence: Vec<u64>,
}

impl Row {
    pub fn new(data: String, sequence: Vec<u64>) -> Self {
        Row { data, sequence}
    }

}
