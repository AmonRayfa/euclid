use std::collections::HashMap;
use serde::{ Serialize, Deserialize }; // Used to deserialize the JSON data into a struct.


// Defines the data structure to hold the JSON data
#[derive(Serialize, Deserialize, Debug)]
pub struct Cache {
    pub freq: usize,
    pub target: String,
    pub name: String,
    pub min: usize,
    pub max: usize,
    pub seq: usize,
    pub same: usize,
    pub case: bool,
    pub index: usize,
    pub sets: HashMap<String, String>, //* We can't use `&'static str` because upon changing the key-value pairs in a `for` loop, the compiler will complain about the lifetime of the string being too short.
}
