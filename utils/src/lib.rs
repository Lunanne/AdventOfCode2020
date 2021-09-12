// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines(file: &str) -> Vec<&str> {
    return file.split_whitespace().collect();
}
