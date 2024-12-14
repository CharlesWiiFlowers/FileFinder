use clap::{Parser, Subcommand};
use commands::find_file;
use std::{
    io::Write,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

// This is a macro
#[derive(Parser)]
#[command(about = "A toolbox of utilities", long_about = "")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Find {
        #[clap(short, long)]
        filename: String,

        #[clap(short, long, default_value = "C://")]
        root: String,
    },
}

fn main() {
    // Time elapsed
    let now = Instant::now();

    // Spinner section
    // This gonna send a stop signal
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

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
            print!(
                "\r{} Loading... {}",
                spinner_chars[i % spinner_chars.len()],
                spinner_chars[i % spinner_chars.len()]
            );
            i += 1;

            std::io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // Main Section

    let cli = Cli::parse();

    match cli.command {
        Commands::Find { filename, root } => {
            match find_file(&root, &filename) {
                Some(paths) => {
                    // Send the STOP SIGNAL to the thread
                    tx.send(()).unwrap();
                    print!("\r");
                    for path in paths {
                        println!("{path}");
                    }

                    println!(
                        "Work finished!! 🐾\nTranscurred time: {} ms",
                        now.elapsed().as_millis()
                    );

                    //Let it finish
                    let _ = spinner_handle.join();
                }
                None => {
                    tx.send(()).unwrap();
                    println!(
                        "\nNo matches found!! 🚀\nTranscurred time: {} ms",
                        now.elapsed().as_millis()
                    );
                    let _ = spinner_handle.join();
                }
            }
        }
    }
}

// TXG
