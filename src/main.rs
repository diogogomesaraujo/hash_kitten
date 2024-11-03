use std::fs;
use std::env;
use std::io;

mod funcs;

const HELP: &str = "  ／l、             
（ﾟ､ ｡ ７       
  l  ~ヽ       
  じしf_,)ノ\n\n\
Welcome to hashkitten! 🐾\n\
Your purr-fect hashing companion.\n\n\
Usage: hashkitten [-h] [-f FILE] [TEXT]\n\
A fun tool for hashing text or files.\n\n\
ARGUMENTS\n\
    -h | --help: Print help and exit\n\
    -f | --file FILE: Specify a file to hash\n\
    TEXT: Input text to be hashed without a flag\n\n\
EXAMPLES\n\
    hashkitten -h                # Display help\n\
    hashkitten -f input.txt      # Hash the contents of input.txt\n\
    hashkitten \"Hello, world!\"   # Hash the given text\n";


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
            eprintln!("An error ocurred while reading the file meow: {}", e);
            Err(e)
        }
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("You didn't give the kitten all the arguments!");
        return;
    }

    if args.len() > 3 || (args[1] == "-f" && args.len() != 3) {
        println!("You gave the kitten too many or too few arguments!");
        return;
    }

    let flag = &args[1].to_string();

    let content:String = match flag.as_str() {
        "-f" => {
            let file_path = &args[2];

            match read_file(file_path){
                Ok(val) => val,
                Err(_) => {
                    eprintln!("The kitten doens't recognize that path!"); 
                    return;
                },
            }
        },
        "-h" =>{
            println!("{}", HELP);
            return;
        }
        _ => {
            flag.clone()
        }
    };

    println!("{}", meow_message(content));
}
