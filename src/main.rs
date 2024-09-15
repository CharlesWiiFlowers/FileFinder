use std::{fs, io::Write, path::Path, sync::mpsc, thread, time::Duration};
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
    // Spinner section
    // This gonna send a stop signal
    // mpsc = multiple producer, single consumer
    let(tx, rx) = mpsc::channel();

    let spinner_handle = thread::spawn(move || {

        // Vector with the future emoji spinner
        let spinner_chars: Vec<&str> = vec!["🌕", "🌖", "🌗", "🌘", "🌑", "🌒", "🌓", "🌔"];
        let mut i = 0;

        loop {
            // did I receive a stop sign?
            if rx.try_recv().is_ok() {
                break;
            }

            // So print the currecnt char of the spinner
            // the index will be the result of divide de cycle number upper the spinner_chars lenght
            print!("\r{} Loading... {}", spinner_chars[i % spinner_chars.len()], spinner_chars[i % spinner_chars.len()]);
            i+=1;

            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200))
        }
        
    });

    // Main Section
    let args = Cli::parse();
    let divided_filename = divide(&args.filename.to_string());

    match search(&args.root.to_string(), &args.filename.to_string()) {
        Some(paths) => {
            // Send the STOP SIGNAL to the thread
            tx.send(()).unwrap();
            print!("\r");
            for path in paths {
                // Sender will be SEND a stop signal
                println!("Founded: {}", path);
            }
            
            //Let it finish
            let _ = spinner_handle.join();
        }
        None => {
            tx.send(()).unwrap();
            println!("No matches found!!");
            let _ = spinner_handle.join();
        }
    }

}

fn divide(filename: &str) -> [String; 3] {
    let mut flag: bool = true;
    let mut flag2:bool = true;

    let mut divided_filename: [String; 3] = [String::new(), String::new(), String::new()];

    for x in filename.chars().rev() {
        if x != '.' && flag == true {
            divided_filename[3] = divided_filename[3].clone() + &x.to_string();
        } else if x == '.' {
            flag = false;
        } else if x != '\\' && flag2 == true && flag == false{
            divided_filename[2] = divided_filename[2].clone() + &x.to_string();
        } else if x == '\\' {
            flag2 = false;
        } else {
            divided_filename[1] = divided_filename[1].clone() + &x.to_string();
        }
    }

    return divided_filename;
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
