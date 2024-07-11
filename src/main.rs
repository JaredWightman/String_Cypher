// Jared Wightman

use std::io;
use std::fs;

// Symbol object for pairing up old and new values
struct Symbol {
    original: u8,
    mod_one: u8,
    mod_two: u8,
}
// BELOW: Methods for printing Symbol info and a failed toggle function (needs to be called mutably)
// impl Symbol {
//     fn print(&self) {
//         println!("Original: {} ({})", self.original, self.original as char);
//         println!("    One: {} ({})", self.mod_one, self.mod_one as char);
//         println!("    Two: {} ({})", self.mod_two, self.mod_two as char);
//     }
//     // fn swap(&mut self) -> u8 {
//     //     let temp = self.mod_one;
//     //     self.mod_one = self.mod_two;
//     //     self.mod_two = temp;
//     //     self.mod_one
//     // }
// }


// Function for creating Symbol object
fn init_symbol(original: u8, mod_one: u8, mod_two: u8) -> Symbol {
    Symbol {
        original,
        mod_one,
        mod_two,
    }
}

// Function that assigns 1 or 2 swap values to an existing symbol, returns vector of Symbol objects
fn create_key(seed:i32) -> Vec<Symbol> {

    // Creating lists of old symbols, new symbols, and Symbol objects
    let mut og_list: Vec<u8> = vec![];
    let mut mod_list: Vec<u8> = vec![];
    let mut symbol_list: Vec<Symbol> = vec![];

    // Loading ASCII values into selector lists
    for count in 32..=126 {
        if !(count >= 65 && count <= 90)  {
            og_list.push(count as u8);
        }
        mod_list.push(count as u8);
    }

    // Setting up symbols, defining swap values. Iterates through vector of possible swaps and pulls them out for each symbol.
    let mut index = seed;
    for count in 0..(og_list.len()-1) {
        // Best seeded random selection algorithm ever made
        let size = mod_list.len() as i32;
        while index > size -1 {
            index = index - size
        }
        let bookmark = index;
        index = index + mod_list[index as usize] as i32;
        while index > size -1 {
            index = index - bookmark - size -3;
        }
        index = index + seed;
        while index < 0 {
            index = index + size
        }
        while index > size -1 {
            index = index - 1;
        }

        let algo_num_1:u8 = mod_list.remove(index as usize);
        let mut algo_num_2:u8 = algo_num_1;

        // If symbol is a letter, assigns a second swap
        if count >= 38 && count <= 64 {

            let size = mod_list.len() as i32;
            index = seed - size;
            while index > size-1 {
                index = index - size;
            }
            index = index + bookmark;
            while index > size -1 {
                index = index - 6;
            }
            while index < 0 {
                index = index + 3
            }
            while index > size -1 {
                index = index - 1;
            }

            algo_num_2 = mod_list.remove(index as usize);
        }

        symbol_list.push(init_symbol(og_list[count], algo_num_1, algo_num_2));
    }
    symbol_list
}


// Encoding Function
fn encode_string(og_string: String, symbol_list:Vec<Symbol>) -> String {

    // BELOW: Failed direct-replacement swapping. Functions, but cannot allow for alternation within the .map segment. (add if-statements enclosed by brackets?)
    // let direct_mod_string:String = og_string.to_ascii_lowercase().chars().rev()
    //     .map(|char| {
    //             match symbol_list.iter().find(|symbol| symbol.original as char == char) {
    //                 //Some(symbol) => symbol.mod_one as char,
    //                 Some(symbol) => symbol.mod_one as char,
    //                 None => char,
    //             }
    //     }
    //     ).collect::<String>();

    let mut mod_string = String::new();
    let mut alternator = 1;

    // Reverses and lowercases all symbols, then swaps to alternate values, switching every other (which only affects symbols with 2 options)
    for char in og_string.to_ascii_lowercase().chars().rev() {
        match symbol_list.iter().find(|symbol| symbol.original as char == char) {
            Some(symbol) => {
                if alternator == 1 {
                    mod_string.push(symbol.mod_one as char);
                    alternator = alternator + 1;
                } else if alternator == 2 {
                    mod_string.push(symbol.mod_two as char);
                    alternator = alternator - 1;
                }
            },
            None => mod_string.push(char),
        }
    }
    mod_string
}


// Decoding function
fn decode_string(dec_string: String, symbol_list:Vec<Symbol>) -> String {

    let mut mod_string = String::new();

    // Checks each swapped symbol, converts to original
    for char in dec_string.chars().rev() {
        match symbol_list.iter().find(|symbol| symbol.mod_one as char == char ||symbol.mod_two as char == char) {
            Some(symbol) => mod_string.push(symbol.original as char),
            None => mod_string.push(char),
        }
    }
    mod_string
}



fn main() {

    let mut seed:i32 = 4;
    let mut enc_string = String::new();
    let mut enc_contents = String::new();

    println!("1 - set seed\n2 - encode input\n3 - decode input\n4 - encode contents of 'message' file\n5 - decode contents of 'message' file\n ");

    // Terminal logic. Gets selection option, checks it, does it.
    loop {

        println!("\n-------------------------------\nEnter a selection: (seed: {seed})");
        let mut selection= String::new();
        let mut user_input = String::new();
        io::stdin().read_line(&mut selection)
            .expect("Input failed");
        match selection.trim().to_ascii_uppercase().as_str() {
            "1" => {
                // Set new seed
                println!("\nEnter a number for the seed:");
                io::stdin().read_line(&mut user_input)
                    .expect("Input failed");
                seed = user_input.trim().parse::<i32>()
                    .expect("Number invalid");
            },
            "2" => {
                // Gets string input and encrypts
                println!("\nEnter a string:");
                io::stdin().read_line(&mut user_input)
                    .expect("Input failed");
                enc_string = encode_string(user_input.clone(), create_key(seed));
                println!("\nEncoded string: {}",enc_string.trim());
            },
            "3" => {
                // Outputs encrypted string
                println!("\nEncoded string: {}",enc_string.trim());
                print!("Decoded string: {}",decode_string(enc_string.clone(), create_key(seed)));
            },
            "4" => {
                // Encrypts and displays contents in file "Message.txt"
                let file_contents = fs::read_to_string("C:/Users/JWigh/RustroverProjects/String_Cypher/Message.txt").unwrap().to_string();
                enc_contents = encode_string(file_contents, create_key(seed));
                println!("\nEncoded message: \n\n{}",enc_contents);
            },
            "5" => {
                // Decrypts and displays previously encrypted file contents
                println!("\nDecoded message: \n\n{}",decode_string(enc_contents.clone(), create_key(seed)));
            },
            // Exit statement
            "E" => break,
            // Default statement
            _ => println!("\nSelection invalid. Please try again."),
        }
    }
}
