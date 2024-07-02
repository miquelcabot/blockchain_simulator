// Blockchain Simulation Program

use sha2::{Digest, Sha256};
use std::fmt;
use std::iter;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// Define the difficulty of mining (number of leading zeros required in hash)
const DIFFICULTY: usize = 2;

// Define the structure of a block in the blockchain
struct Block {
    index: u32,            // Index of the block in the chain
    previous_hash: String, // Hash of the previous block
    timestamp: u64,        // Timestamp of block creation
    data: String,          // Data stored in the block
    nonce: u64,            // Nonce used for mining
    hash: String,          // Hash of the current block
}

impl Block {
    // Constructor for creating a new block
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        // Get the current timestamp in seconds since UNIX epoch
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        Block {
            index,               // Set the index
            previous_hash,       // Set the previous hash
            timestamp,           // Set the timestamp
            data,                // Set the data
            nonce: 0,            // Initialize nonce to 0
            hash: String::new(), // Initialize hash to empty string
        }
    }

    // Method to calculate the hash of the block
    fn calculate_hash(&mut self) -> String {
        // Concatenate block attributes into a single string
        let data = format!(
            "{}{}{}{}{}", // Format string with index, previous_hash, timestamp, data, nonce
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
        );

        // Create a SHA-256 hasher and update it with block data
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        // Format the hash result as a hexadecimal string and return
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

// Implementing formatting for Block structure to allow printing
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

fn main() {
    println!("Welcome to CABOTCOIN Mining Simulator!");
}
