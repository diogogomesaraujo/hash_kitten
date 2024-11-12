use std::fs;
use std::fs::File;
use std::env;
use std::io;
use std::io::Write;

mod funcs;

const HELP: &str = "\n  ／l、             
（ﾟ､ ｡ ７       
  l  ~ヽ       
  じしf_,)ノ\n\n\
Welcome to hashkitten! 🐾\n\
Your purrfect hashing companion.\n\n\
Usage: hashkitten [-h] [-f FILE] [-c FILE|\"TEXT\" HASH] [\"TEXT\"]\n\
A fun tool for hashing text or files.\n\n\
ARGUMENTS\n\
    -h | --help: Print help and exit\n\
    -f | --file FILE [OUTPUT]: Hash the contents of the specified file. Optionally, write the hash to OUTPUT.\n\
    -c | --compare FILE|\"TEXT\" HASH: Compare the contents of a file or a message to the specified hash.\n\
    \"TEXT\": Input text to be hashed (must be enclosed in double quotes).\n\n\
EXAMPLES\n\
    hashkitten -h                        # Display help\n\
    hashkitten -f input.txt              # Hash the contents of input.txt\n\
    hashkitten -f input.txt output.txt   # Hash the contents of input.txt and write the hash to output.txt\n\
    hashkitten -c input.txt HASH         # Compare the hash of input.txt with HASH\n\
    hashkitten -c \"Hello, world!\" HASH  # Compare the hash of the message with HASH\n\
    hashkitten \"Hello, world!\"          # Hash the given text (must be in quotes)\n";

fn meow_message(message: String) -> String{
    let message_bytes = funcs::pre_processing(message);
    let message_schedule = funcs::create_message_schedule(message_bytes);
    let hash_values = funcs::compressing_schedule(message_schedule);
    let sha256_hash = funcs::concatenate_hash(hash_values);
    let meow = funcs::meow_from_hash(&sha256_hash);

    meow
}

fn read_file(file_path: &String) -> Result<String, io::Error>{
    match fs::read_to_string(file_path){
        Ok(val) => Ok(val),
        Err(e) => {
            Err(e)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("*confused meow* No arguments provided! Use `-h` or `--help` for usage instructions.");
        return;
    }

    match args.get(1).map(|s| s.as_str()) {
        Some("-h") | Some("--help") => {
            println!("{}", HELP);
        }

        Some("-f") | Some("--file") => {
            if args.len() < 3 {
                eprintln!("*confused meow* Missing file path! Use `-f FILE` to specify a file.");
                return;
            }
            let file_path = &args[2];
            let output = args.get(3).map(String::from);
            match read_file(file_path) {
                Ok(contents) => {
                    let hash = meow_message(contents);
                    if let Some(output_path) = output {
                        match File::create(&output_path) {
                            Ok(mut file) => {
                                if let Err(e) = file.write_all(hash.as_bytes()) {
                                    eprintln!("*surprised meow* Failed to write hash to file: {}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("*surprised meow* Failed to create file: {}", e);
                            }
                        }
                    } else {
                        println!("{}", hash);
                    }
                }
                Err(e) => {
                    eprintln!("*confused meow* Could not read file: {}", e);
                }
            }
        }

        Some("-c") | Some("--compare") => {
            if args.len() < 4 {
                eprintln!("*confused meow* Missing arguments for comparison! Use `-c \"MESSAGE\" HASH` or `-c FILE HASH`.");
                return;
            }

            let input = args[2].clone();
            let hash = args[3].clone();

            let message = match read_file(&input) {
                Ok(contents) => contents,
                Err(_) => input,
            };

            let calculated_hash = meow_message(message);

            if calculated_hash == hash {
                println!("*excited purrs* Meow meow! The hash matches purrfectly! 🐱✨");
            } else {
                println!("*sad meow* The hash doesn't match! 🐱💔");
            }
        }

        Some(_) => {
            let message = args[1..].join(" ");
            println!("{}", meow_message(message));
        }

        None => {
            eprintln!("*confused meow* No valid arguments provided! Use `-h` for help.");
        }
    }
}
