use std::path::Path;
use std::fs;
use clap::Parser;

// This is a macro
#[derive(Parser)]
#[command(name = "search")]
#[command(
    about = "Search a specific file in your system",
    long_about = "This request a name and verify
 your files on your system."
)]
struct Cli {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value = "C://")]
    root: String,
}

fn main() {
    let args = Cli::parse();

    match search(&args.root.to_string(), &args.filename.to_string()) {
        Some(paths) => {
            for path in paths {
                println!("Founded: {}", path)
            }
        }
        None => {
            println!("No matches found!!")
        }
    }

}

// Search a name in a dir
fn search(dir: &str, filename: &str) -> Option<Vec<String>> {
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
                        search(entry_path.to_str().unwrap(), filename)
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

// TXG