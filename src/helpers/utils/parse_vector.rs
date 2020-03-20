pub mod vector {
    pub fn vector_from_string (value: String) -> Vec<String> {
        value
            .trim()
            .split(',')
            .map(String::from)
            .collect::<Vec<String>>()
    }
}
