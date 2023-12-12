use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lazy_static::lazy_static;
use std::collections::HashMap;

// Define a global-like variable using lazy_static
lazy_static! {
    static ref CUBE_LIMIT: HashMap<&'static str, i32> = {
        let mut map = HashMap::new();
        // Initialize your key-value pairs here
        map.insert("blue", 14);
        map.insert("red", 12);
        map.insert("green", 13);
        map
    };
}

fn is_valid_game(key_value_pairs: HashMap<&str, i32>) -> bool 
{
    // Process the key-value pairs as needed
    let mut ret_val = true;
    'checkloop:for (key, value) in key_value_pairs {
        if let Some(&ref_val) = CUBE_LIMIT.get(key) {
            // Use the value without cloning
            // println!("Value for key '{}' is: {}", key, value);
            if value > ref_val
            {
                ret_val = false;
                break 'checkloop;
            }
        } else {
            println!("Key not found");
        }
    }
    ret_val
}
fn part_one(key_value_pairs:HashMap<&str, i32>, game_id: i32) -> i32
{
    let mut result = 0;
    if is_valid_game(key_value_pairs)
    {
        result += game_id;
    }
    result
}

fn part_two(key_value_pairs:HashMap<&str, i32>) -> i32
{
    let mut result = 1;
    for (key, value) in key_value_pairs
    {
        result *= value;
    }
    result
}

fn main() {
    let file_path = "src/input.txt";
    let mut game_id = 1;
    let mut result = 0;
    if let Ok(file) = File::open(&file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                // Split the line based on the first colon to separate key and value
                let picks = line.split_once(':').unwrap().1.trim();
                // let picks_sep : Vec<&str>= picks.split(';').collect();

                let mut key_value_pairs: HashMap<&str, i32> = HashMap::new();
                for pick in picks.split(';')
                {
                    let pairs: Vec<&str> = pick.split(',').collect();
                    for pair in pairs 
                    {
                        let values: Vec<&str> = pair.trim().split_whitespace().collect();
                        if values.len() == 2 {
                            let key = values[1];
                            let value = values[0].parse::<i32>().unwrap_or_default();
            
                            // Update the key-value pair only if the value is greater
                            if let Some(existing_value) = key_value_pairs.get(key) {
                                if value > *existing_value {
                                    key_value_pairs.insert(key, value);
                                }
                            } else {
                                key_value_pairs.insert(key, value);
                            }
                        }
                    }
                }
                // println!("-----");
                // result += part_one(key_value_pairs, game_id);
                result += part_two(key_value_pairs);
                game_id+=1;
            }
        }
    } else {
        println!("Error opening the file: {}", file_path);
    }
    println!("result is {}", result);
}
