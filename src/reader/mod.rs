use crate::utils;

// A function that reads a file and either returns an Error or the content of the file as string.
pub fn read_file(f_path: &str) -> Result<String, std::io::Error> {
    let path_binding = utils::parse_path(f_path).unwrap();
    let path = path_binding.to_str().unwrap();
    Ok(std::fs::read_to_string(path)?)
}