use std::fs;
use std::env;

const PRIME_NUMBERS: &[i32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311];

const MEOW_MAP: [(char, &str); 36] = [
    ('A', "meow"), ('B', "meoww"), ('C', "meeow"), ('D', "meoow"), 
    ('E', "meowwww"), ('F', "mmeeow"), ('G', "mmmeow"), ('H', "meeeow"),
    ('I', "meowow"), ('J', "mmmmeow"), ('K', "meowww"), ('L', "meeoww"),
    ('M', "meooww"), ('N', "meowwmeow"), ('O', "mmeoww"), ('P', "meowwmeeow"),
    ('Q', "meeowmeow"), ('R', "meeeoow"), ('S', "mmmeoow"), ('T', "mmeeoow"),
    ('U', "meoowww"), ('V', "meeowow"), ('W', "meowmeow"), ('X', "mmmeeow"),
    ('Y', "meeeoww"), ('Z', "mmmmeoow"), 
    ('0', "meowmeow"), ('1', "meowmeoww"),
    ('2', "meeowmeow"), ('3', "meoowmeow"), 
    ('4', "meowwmeow"), ('5', "meowwwwmeow"),
    ('6', "mmmeowmeow"), ('7', "meowmeowow"), 
    ('8', "meowwmeeow"), ('9', "meeeowmeow")
];

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
        
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }

        schedules.push(w);
    }

    schedules
}

fn compressing_schedule(schedule: Vec<Vec<u32>>) -> Vec<u32>{
    let mut hash_values = start_hash_values();
    let round_constants = start_round_constants();

    let mut a = hash_values[0];
    let mut b = hash_values[1];
    let mut c = hash_values[2];
    let mut d = hash_values[3];
    let mut e = hash_values[4];
    let mut f = hash_values[5];
    let mut g = hash_values[6];
    let mut h = hash_values[7];


    for chunk in &schedule{
        for i in 0..64{
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = h.wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(round_constants[i])  // Round constant
                .wrapping_add(chunk[i]); // Current word from the schedule
            
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        hash_values[0] = hash_values[0].wrapping_add(a);
        hash_values[1] = hash_values[1].wrapping_add(b);
        hash_values[2] = hash_values[2].wrapping_add(c);
        hash_values[3] = hash_values[3].wrapping_add(d);
        hash_values[4] = hash_values[4].wrapping_add(e);
        hash_values[5] = hash_values[5].wrapping_add(f);
        hash_values[6] = hash_values[6].wrapping_add(g);
        hash_values[7] = hash_values[7].wrapping_add(h);  
    }

    hash_values
}

fn concatenate_hash(hash_values: Vec<u32>) -> String{
    hash_values.iter().map(|value| format!("{:08x}", value)).collect()
}

fn meow_from_hash(hash: &String) -> String{
    let mut meow = String::new();
    for letter in hash.chars(){
        if let Some(matching_meow) = MEOW_MAP.iter().find_map(|&(c, m)| {
            if c == letter {
                Some(m) // Return `Some(m)` if the character matches
            } else {
                None
            }
        }) {
            meow += matching_meow; // Append the matching value to `meow`
        }
    }

    meow
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

    let hash_values = compressing_schedule(schedule);
    let final_hash = concatenate_hash(hash_values);
    let meow = meow_from_hash(&final_hash);

    println!("Final Hash: {}", final_hash);

    println!("Final Meow: {}", meow)
}
