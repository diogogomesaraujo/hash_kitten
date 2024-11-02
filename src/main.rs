use std::io::{self, Write};

fn pre_processing(message: String) -> Vec<u8>{
    let mut bytes: Vec<u8> = message.into_bytes(); // convert to vetor of bytes
    let mut message_size = bytes.len();

    bytes.push(128 as u8);

    if message_size + 1 + 8 <= 64{ // isize plus the max space the one can ocupy plus the max size of the string
        for _i in 0..(64 - bytes.len() - 8){
            bytes.push(0 as u8);
        }

        message_size *= 8;
        bytes.extend_from_slice(&message_size.to_be_bytes());
    } else{
        let aux_num_block = (message_size + 1 + 8) / 64;
        for _i in 1..aux_num_block{
            bytes.push(0 as u8);
        }

        message_size *= 8;
        bytes.extend_from_slice(&message_size.to_be_bytes());
    }

    bytes
}

fn main() {
    print!("Write a message for the kitten to hash -> ");
    io::stdout().flush().unwrap();

    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read the message!");

    let bytes: Vec<u8> = pre_processing(message.trim_end().to_string());

    for byte in bytes{
        print!("{:08b} ", byte);
        io::stdout().flush().unwrap();
    }
    println!();
}
