use std::fs;

/// Reads all file contents to memory - todo: improve to use buffer
/// * `file_path` - Path to file to read
pub fn read_file_mem(file_path: &str) -> String {
    fs::read_to_string(file_path).expect(format!("Could not read file {:?}", file_path).as_str())
}
