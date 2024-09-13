use clap::Parser;

// This is a macro
#[derive(Parser)]
#[command(name = "search")]
#[command(about = "Search a specific file in your system", long_about = "This request a name and verify
 your files on your system.")]
struct Cli{
    #[arg(short, long, default_value = "World")]
    path: String
}
fn main() {
    let args = Cli::parse();
    println!("Hello, {}!", args.path);
}
