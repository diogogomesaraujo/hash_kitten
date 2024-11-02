use std::fs;
use std::env;

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
    let message_size = bytes.len();

    bytes.push(0b10000000);

    // padd untill 448
    while (bytes.len() % 64) != 56{
        bytes.push(0);
    }

    let message_size = (message_size * 8) as u64;
    bytes.extend_from_slice(&message_size.to_be_bytes());

    bytes
}

fn create_message_schedule(pre_processed_message: Vec<u8>) -> Vec<Vec<u32>>{
    let mut schedules: Vec<Vec<u32>> = vec![];
    for block in pre_processed_message.chunks(64){ // for each 512bit chunk
        let mut w: Vec<u32> = block.chunks(4)
            .map(|chunk| {
                (chunk[0] as u32) << 24 |
                (chunk[1] as u32) << 16 |
                (chunk[2] as u32) << 8  |
                (chunk[3] as u32)
            }).collect();

        w.resize(64, 0); // fill with 0 what is missing
        
        for i in 16..64{
            let s0: u32 = (w[i - 15].rotate_right(7)) ^ (w[i - 15].rotate_right(18)) ^ (w[i-15] >> 3);
            let s1: u32 = (w[i - 2].rotate_right(17)) ^ (w[i - 2].rotate_right(19)) ^ (w[i - 2] >> 10);
        
            w[i] = (w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1));
        }

        schedules.push(w);
    }

    schedules
}

fn main() {
    let args:Vec<String> = env::args().collect();

    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("The kitten could not read the file!");

    let content_bytes = pre_processing(content);
    println!("{:?}\n", content_bytes);

    let schedule = create_message_schedule(content_bytes);
    println!("{:?}\n", schedule);
}
