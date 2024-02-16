use std::path::PathBuf;

// A path parser so Hisoku can be cross-platform and it's also use in the run command and more..
pub fn parse_path(path: &str) -> Option<PathBuf> {
    let mut current_dir = std::env::current_dir().unwrap(); // Start from current directory
    
    for segment in path.split('/') 
    {
        if segment == "." {
            continue; // Skip "."
        } else if segment == ".." {
            current_dir.pop(); // Move up one directory
        } else {
            current_dir.push(segment); // Add segment to path
        }
    }

    Some(current_dir)
}