use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
fn main() {
    // Create a context to interact with the clipboard
    let mut ctx = ClipboardContext::new().expect("Failed to initialize clipboard");

    // Create a new empty HashMap to store keys of type String and values of type i32
    let unique_clipboard_contents: HashMap<String, i32> = HashMap::new();

    loop {
        // Attempt to get the contents of the clipboard
        match ctx.get_contents() {
            Ok(contents) => {
                // Check if a key exists in the HashMap
                if !unique_clipboard_contents.contains_key(&contents) {
                    println!("inserting {contents}");
                    unique_clipboard_contents.insert(contents, 1)
                    println!("{unique_clipboard_contents}");
                }
                else {
                    let count = unique_clipboard_contents.get(contents);
                    unique_clipboard_contents.insert(contents, count+1)
                    println!("{unique_clipboard_contents}");
                }
            }
            Err(err) => {
                // Handle the error if clipboard access fails
                eprintln!("Error getting clipboard contents: {:?}", err);
            }
        }
    }
}


    // // Insert key-value pairs into the HashMap
    // my_hash_map.insert(String::from("apple"), 5);
    // my_hash_map.insert(String::from("banana"), 10);
    // my_hash_map.insert(String::from("orange"), 7);

    // // Access and print the value associated with a specific key
    // match my_hash_map.get("apple") {
    //     Some(quantity) => println!("Number of apples: {}", quantity),
    //     None => println!("No information about apples"),
    // }

    // // Update a value associated with a key
    // my_hash_map.insert(String::from("apple"), 8);

    // // Check if a key exists in the HashMap
    // if !my_hash_map.contains_key("grapes") {
    //     println!("No information about grapes");
    // }

    // // Remove a key-value pair from the HashMap
    // my_hash_map.remove("orange");

    // // Iterate through the HashMap and print all key-value pairs
    // for (fruit, quantity) in &my_hash_map {
    //     println!("{}: {}", fruit, quantity);
    // }
