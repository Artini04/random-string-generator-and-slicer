// RANDOM STRING GENERATOR AND SLICER (by chunks)
// Some codes are from the Rust cookbook (Create random passwords from a set of user-defined characters)
use rand::Rng;

fn main() {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    const PASSCODE_LEN: usize = 25;
    const CHUNK_SIZE: usize = 4;
    let mut rng = rand::thread_rng();

    // Generate passcode
    let passcode: String = (0..PASSCODE_LEN)
        .map(|_| {
            // Randomly select number from 0 to the length of CHARSET
            // then map and build passcode
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    // Slice passcode into CHUNK_SIZE chunks
    let chunks: Vec<String> = passcode
        .chars()
        .collect::<Vec<char>>()
        .chunks(CHUNK_SIZE)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();

    // Print result
    println!("\nInitial code:\n\t[ {} ]\n", passcode.to_string());
    println!(
        "Sliced code: (by {} chunks)\n\t[ {} ] => {}",
        CHUNK_SIZE.to_string(),
        chunks.join(" - "),
        chunks.len().to_string()
    );
}
