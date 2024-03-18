mod cache;
mod menu;
mod cracker;
use serde_json; // Used to serialize the cache struct into a JSON string.


fn main() {
    // TODO: Make a multi-threaded version of the program.
    // TODO: Find a way to deserialize the cache.sets into a HashMap<usize, &str> instead of HashMap<&str, &str>. That way we can use the `k` variable as a key without having to convert it to a string (`.to_string().as_str()`) which is an operation with an O(n) time complexity.
    //? For some strange reason, this version of the program and it's PowerShell equivalent don't produce the same final number of attempts and skips.
    // Opens the cache file, read it into a string and deserialize it into a struct.
    const FILE_PATH: &str = "cache.json";
    let cache_string = std::fs::read_to_string(FILE_PATH).unwrap();
    let mut cache: crate::cache::Cache = serde_json::from_str(&cache_string).expect("Error while reading json");

    crate::menu::menu(&FILE_PATH, &mut cache, &mut false, &mut "".to_string(), &mut "", &mut "".to_string());
    crate::cracker::cracker(&mut cache);
}
