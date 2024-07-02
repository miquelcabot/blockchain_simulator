use sha2::{Digest, Sha256};
use std::fmt;
use std::iter;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Define difficulty of the mining process
const DIFFICULTY: usize = 2;

// Define the structure of a block in the blockchain
struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    data: String,
    nonce: u64,
    hash: String,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str = format!("{:x}", result);
        hash_str
    }

    // Method to mine the block with visual effects
    fn mine_block_with_visual_effects(&mut self) {
        let mut iterations = 0; // Initialize iterations counter
        loop {
            self.hash = self.calculate_hash(); // Calculate the hash of the block
            iterations += 1; // Increment iterations counter
                             // Check if the hash meets the difficulty requirement
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                // Print a message indicating successful block mining
                println!("⛏️ Block mined: {}", self.index);
                break; // Exit the loop
            }
            // If the iterations exceed a certain limit, print the calculated hash and pause for visual effect
            if iterations > 100 {
                print!("⏳ Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }
            self.nonce += 1; // Increment the nonce for the next iteration
        }
    }
}

fn main() {
    println!("Welcome to CABOTCOIN Mining Simulator!");
}
