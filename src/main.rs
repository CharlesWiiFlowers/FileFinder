use clap::Parser;

// This is a macro
#[derive(Parser)]
#[command(name = "search")]
#[command(about = "Search a specific file in your system", long_about = "This request a name and verify
 your files on your system.")]
struct Cli{
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value = "C://")]
    root: String
}

fn divide(filename: &str) -> (String, String) {

    let _ = filename;
    let mut file: String = String::from("");
    let mut ext: String = String::from("");
    let mut flag:bool = true;

    // Include 0 and exclude lenght
    for x in filename.chars().rev() {
        
        if x != '.' && flag == true {
            ext = ext + &x.to_string();
        } else if x == '.'{
            flag = false;
        } else {
            file = file + &x.to_string();
        }
        }

    let tuple: (String, String) = (file, ext);
    return tuple;

    }

fn main() {
    let args = Cli::parse();

    let _: (String, String) = divide(&args.filename.to_string());

}