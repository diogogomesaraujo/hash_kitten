use std::fs;
use std::env;
use std::io;

mod funcs;

const HELP: &str = "  ／l、             
（ﾟ､ ｡ ７       
  l  ~ヽ       
  じしf_,)ノ\n\n\
Welcome to hashkitten! 🐾\n\
Your purrfect hashing companion.\n\n\
Usage: hashkitten [-h] [-f FILE] [\"TEXT\"]\n\
A fun tool for hashing text or files.\n\n\
ARGUMENTS\n\
    -h | --help: Print help and exit\n\
    -f | --file FILE: Specify a file to hash\n\
    -c | --compare \"MESSAGE\" HASH: Specify a message and a hash to compare\n\
    \"TEXT\": Input text to be hashed (must be enclosed in double quotes)\n\n\
EXAMPLES\n\
    hashkitten -h                # Display help\n\
    hashkitten -f input.txt      # Hash the contents of input.txt\n\
    hashkitten -c \"Hello, world!\" HASH # Compare the message to the given hash\n\
    hashkitten \"Hello, world!\"     # Hash the given text (must be in quotes)\n";

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
    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Meow? *tilts head* I need more arguments to play with! 🐱");
        return;
    }

    if args.len() > 4 || (args[1] == "-f" && args.len() != 3)  || (args[1] == "-c" && args.len() != 4){
        println!("Meow? *confused purring* Too many or too few things to play with! The kitten is confused! 🐱");
        return;
    }

    let flag = &args[1].to_string();

    let content:String = match flag.as_str() {
        "-f" => {
            let file_path = &args[2];

            match read_file(file_path){
                Ok(val) => val,
                Err(_) => {
                    eprintln!("*confused meow* The kitten doesn't recognize that path! Purrhaps try a different one?🐱");                    
                    return;
                },
            }
        },
        "-h" =>{
            println!("{}", HELP);
            return;
        },
        "-c" =>{
            let message: String = args[2..(args.len() - 1)].join(" ");
            let hash: String = args[args.len() - 1].to_string();

            if meow_message(message) == hash{
                println!("*excited purrs* Meow meow! The hash matches purrfectly! This kitten can confirm they're the same! 🐱✨");
                return;
            } else{
                println!("*sad meow* The hash doesn't match! This kitten can tell they're different... Purrhaps there was a mistake? 🐱💔");
                return;
            }
        }
        _ => {
            args[1..].join(" ")
        }
    };

    println!("{}", meow_message(content));
}