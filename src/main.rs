use std::io::{self, Write};

const PRIME_NUMBERS: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311];

fn start_hash_values() -> Vec<u32>{
    let mut hash_values: Vec<u32> = vec![];

    for prime in &PRIME_NUMBERS[0..8]{
        let aux: f64 = (*prime as f64).sqrt().fract();
        hash_values.push((aux * (2.0f64.powi(32))) as u32); // scale the number to 32bit
    }

    hash_values
}

fn start_round_constants() -> Vec<u32>{
    let mut round_constants: Vec<u32> = vec![];

    for prime in PRIME_NUMBERS{
        let aux: f64 = (*prime as f64).cbrt().fract();
        round_constants.push((aux * (2.0f64.powi(32))) as u32);
    }

    round_constants
}

fn pre_processing(message: String) -> Vec<u8>{
    let mut bytes: Vec<u8> = message.into_bytes(); // convert to vetor of bytes
    let mut message_size = bytes.len();

    bytes.push(128 as u8);

    if message_size + 1 + 8 <= 64{ // isize plus the max space the one can occupy plus the max size of the string
        for _i in 0..(64 - bytes.len() - 8){ // iterate through all the 0 space
            bytes.push(0 as u8);
        }

        message_size *= 8;
        bytes.extend_from_slice(&message_size.to_be_bytes());
    } else{
        let aux_num_block = (message_size + 1 + 8) / 64; // get the size in 512 base
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
