use dirs::data_dir;
use std::io::prelude::*;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

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
                    if let Some(mut subdir_matches) =
                        find_file(entry_path.to_str().unwrap(), filename)
                    {
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

/* End to Find command, I'm begginer in Rust yet */
/* The Tag's kingdom begins!! */

fn verify_folder() -> Option<PathBuf> {
    /* Verify if a tag.json already exists on %appdata%/heimdall
    The name is probably temporal but Heimdall is a perfect name for this project cuz his awesome sight
    (He can "see" the files and invoke it using a tag B)*/

    if let Some(mut app_path) = data_dir() {
        app_path.push("Heimdall");

        // The folder already exist, right???
        if !app_path.exists() {
            if let Err(exception) = fs::create_dir_all(&app_path) {
                eprint!(
                    "💀 Error to created folder on {} 💀\n💀 Error: {} 💀",
                    app_path.display(),
                    exception
                );
                return None;
            }
        }

        app_path.push("tag.json");
        // The path already exist, right???
        if app_path.exists() {
            Some(app_path)
        } else {
            println!(
                "🦈 Tag.json don't exist!! 🦈\n🐳 Creating on {} 🐳",
                app_path.display()
            );
            // Untested: Create a tag.json on %appdata%/heimdall
            match File::create(&app_path) {
                Ok(mut file) => {
                    file.write_all(b"{}").ok()?;
                    None
                }
                Err(error) => {
                    eprintln!(
                        "💀 Error when tried to create a file on {} 💀\n💀 Error: {} 💀",
                        app_path.display(),
                        error.to_string()
                    );
                    return None;
                }
            }
        }
    } else {
        panic!("💀 Can't to find the Heimdall's data directory 💀");
    }
}

pub fn add_tag_file(_dir: &str, _tag: &str) {
    let _ = verify_folder();
}
// TXG
