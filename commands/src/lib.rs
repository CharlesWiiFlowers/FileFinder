use std::{fs, path::Path};

// Find Command: Search a name in a dir
pub fn find_file(dir: &str, filename: &str) -> Option<Vec<String>> {
    let mut matching_files = Vec::new();
    let path = Path::new(dir);

    // see if result when I read the dir is Ok
    if let Ok(entries) = fs::read_dir(path) {
        // For each entry in entries
        for entry in entries {
            // If entry returns Ok
            if let Ok(entry) = entry {
                let entry_path = entry.path();

                if entry_path.is_dir() {
                    if let Some(mut subdir_matches) = find_file(entry_path.to_str().unwrap(), filename) {
                        matching_files.append(&mut subdir_matches);
                    }
                } else if let Some(file_name) = entry_path.file_name() {
                    let file_name_str = file_name.to_string_lossy();
                    if file_name_str.contains(filename) {
                        matching_files.push(entry_path.display().to_string());
                    }
                }
            }
        }
    }

    if matching_files.is_empty() {
        None
    } else {
        Some(matching_files)
    }
}

// TXG