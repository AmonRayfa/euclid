use serde::{ Serialize, Deserialize }; // Used to deserialize the JSON data into a struct.
use colored::*; // Used to color the output.
use serde_json::{ self, to_string_pretty }; // Used to serialize the cache struct into a JSON string.

use std::collections::HashMap;
use std::process::exit; // Used to exit the program.
use std::path::Path; // Used to check if a file exists.
use std::fs::{ File, write }; // Used to open and write to a file.
use std::io::{ self, Read, Write }; // Used to read the contents of a file into a String (see `read_to_string` method).


// Defines the data structure to hold the JSON data
#[derive(Serialize, Deserialize, Debug)]
struct Cache {
    freq: usize,
    target: String,
    name: String,
    min: usize,
    max: usize,
    seq: usize,
    same: usize,
    case: bool,
    index: usize,
    sets: HashMap<String, String> //* We can't use `&'static str` because upon changing the key-value pairs in a `for` loop, the compiler will complain about the lifetime of the string being too short.
}


// Menu function
fn menu(file_path: &str, cache: &mut Cache, hide: &mut bool, message: &mut String, color: &mut &str, prompt: &mut String) {
    
    if !*hide {
        println!("\n{}", " /######$   /######  /##   /##     /## /######$  /###### /##   /##  /###### ".green());
        println!("{}", "| ##__  ## /##__  ##| ##  |  ##   /##/| ##__  ##|_  ##_/| ##  | ## /##__  ##".green());
        println!("{}", "| ##  \\ ##| ##  \\ ##| ##   \\  ## /##/ | ##  \\ ##  | ##  | ##  | ##| ##  \\__/".green());
        println!("{}", "| ######$/| ##  | ##| ##    \\  ####/  | ######$   | ##  | ##  | ##|  ###### ".green());
        println!("{}", "| ##____/ | ##  | ##| ##     \\  ##/   | ##__  ##  | ##  | ##  | ## \\____  ##".green());
        println!("{}", "| ##      | ##  | ##| ##      | ##    | ##  \\ ##  | ##  | ##  | ## /##  \\ ##".green());
        println!("{}", "| ##      |  ######/| ########| ##    | #######/ /######|  ######/|  ######/".green());
        println!("{}\n\n", "|__/       \\______/ |________/|__/    |_______/ |______/ \\______/  \\______/ ".green());

        println!("Display frequency:          freq={}", cache.freq.to_string().as_str().blue());
        println!("Target password:            target={}", cache.target.blue());
        println!("Name of the cache:          name={}\n", cache.name.blue());

        println!("Minimum length:             min={}", cache.min.to_string().as_str().magenta());
        println!("Maximum length:             max={}\n", cache.max.to_string().as_str().magenta());

        println!("Sequential characters:      seq={}", cache.seq.to_string().as_str().cyan());
        println!("Same characters:            same={}", cache.same.to_string().as_str().cyan());
        println!("Case-sensitive:             case={}\n", cache.case.to_string().as_str().cyan());

        print!("Character index:            index=(");
        for i in 1..cache.max {
            if i==cache.index { print!(" {} ", "*".yellow()) }
            else { print!(" * ") }
        }
        if cache.max==cache.index { println!(" {} )", "*".yellow()) }
        else { println!(" * )") }
        println!("Character sets:             sets=[{}]", cache.sets[cache.index.to_string().as_str()].yellow());
    }

    println!("");
    if *message != "" {
        if *color == "red" { println!("{}\n", message.red()) }
        else if *color == "green" { println!("{}\n", message.green()) }
        else if *color == "yellow" { println!("{}\n", message.yellow()) }
        else if *color == "blue" { println!("{}\n", message.blue()) }
        else if *color == "magenta" { println!("{}\n", message.magenta()) }
        else if *color == "cyan" { println!("{}\n", message.cyan()) }
        else { println!("{}\n", message) }
    }

    *hide = false;
    *message = "".to_string();
    *color = "white";
    *prompt = "".to_string();

    print!("({}){} ", "prompt".red(), "$".green());
    io::stdout().flush().unwrap();
    io::stdin().read_line(prompt).expect("Failed to read line");
    *prompt = prompt.trim().to_string();

    if prompt.starts_with("freq") {
        if prompt == "freq" {
            *hide = true;
            *message = "The frequency at which the program displays the number of attempts and skips.".to_string();
            *color = "white";
        } else if prompt.starts_with("freq=") {
            if !prompt[5..].parse::<usize>().is_ok() {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be an integer greater than 0.";
                *color = "red";
            } else {
                cache.freq = prompt[5..].parse::<usize>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("target") {
        if prompt == "target" {
            *hide = true;
            *message = "The target of the brute force algorithm. The value must be 'user' or 'zip'.".to_string();
            *color = "white";
        } else if prompt.starts_with("target=") {
            if &prompt[7..] != "user" && &prompt[7..] != "zip" {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[7..];
                *message += "'. The value must be 'user' or 'zip'.";
                *color = "red";
            } else {
                cache.target = prompt[7..].to_string();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("name") {
        if prompt == "name" {
            *hide = true;
            *message = "The name of the target. The value must be a valid username or a valid path to a zip file depending on the target.".to_string();
            *color = "white";
        } else if prompt.starts_with("name=") {
            if &prompt[5..] == "" {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be a valid username or a valid path to a zip file depending on the target.";
                *color = "red";
            } else if cache.target == "zip" && (!Path::new(&prompt[5..]).exists() || !prompt[5..].ends_with(".zip")) {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be a valid path to a zip file.";
                *color = "red";
            } else {
                cache.name = prompt[5..].to_string();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("min") {
        if prompt == "min" {
            *hide = true;
            *message = "The minimum length of the password.".to_string();
            *color = "white";
        } else if prompt.starts_with("min=") {
            if !prompt[4..].parse::<usize>().is_ok() || prompt[4..].parse::<usize>().unwrap() > 16 {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[4..];
                *message += "'. The value must be an integer greater than 0 and less than 17.";
                *color = "red";
            } else {
                if cache.max < prompt[4..].parse::<usize>().unwrap() { cache.max = prompt[4..].parse::<usize>().unwrap() }
                cache.min = prompt[4..].parse::<usize>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("max") {
        if prompt == "max" {
            *hide = true;
            *message = "The maximum length of the password.".to_string();
            *color = "white";
        } else if prompt.starts_with("max=") {
            if !prompt[4..].parse::<usize>().is_ok() || prompt[4..].parse::<usize>().unwrap() > 16 {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[4..];
                *message += "'. The value must be an integer greater than 0 and less than 17.";
                *color = "red";
            } else {
                if prompt[4..].parse::<usize>().unwrap() < cache.min { cache.min = prompt[4..].parse::<usize>().unwrap() }
                if prompt[4..].parse::<usize>().unwrap() < cache.index { cache.index = prompt[4..].parse::<usize>().unwrap() }
                cache.max = prompt[4..].parse::<usize>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("seq") {
        if prompt == "seq" {
            *hide = true;
            *message = "The maximum number of sequential characters in every attempted password.".to_string();
            *color = "white";
        } else if prompt.starts_with("seq=") {
            if !prompt[4..].parse::<usize>().is_ok(){
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[4..];
                *message += "'. The value must be an integer greater than or equal 0.";
                *color = "red";
            } else {
                cache.seq = prompt[4..].parse::<usize>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("same") {
        if prompt == "same" {
            *hide = true;
            *message = "The maximum number of same characters in every attempted password.".to_string();
            *color = "white";
        } else if prompt.starts_with("same=") {
            if !prompt[5..].parse::<usize>().is_ok(){
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be an integer greater than or equal 0.";
                *color = "red";
            } else {
                cache.same = prompt[5..].parse::<usize>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("case") {
        if prompt == "case" {
            *hide = true;
            *message = "Whether or not the password is case-sensitive.".to_string();
            *color = "white";
        } else if prompt.starts_with("case=") {
            if &prompt[5..] != "true" && &prompt[5..] != "false" && &prompt[5..] != "True" && &prompt[5..] != "False" {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be 'true' or 'false'.";
                *color = "red";
            } else {
                cache.case = prompt[5..].parse::<bool>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("index") {
        if prompt == "index" {
            *hide = true;
            *message = "The index of the current character.".to_string();
            *color = "white";
        } else if prompt.starts_with("index=") {
            if !prompt[6..].parse::<usize>().is_ok() || prompt[6..].parse::<usize>().unwrap() > cache.max {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[6..];
                *message += "'. The value must be an integer greater than 0 and less than or equal to the maximum length of the password.";
                *color = "red";
            } else {
                cache.index = prompt[6..].parse::<usize>().unwrap();
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt.starts_with("sets") {
        if prompt == "sets" {
            *hide = true;
            *message = "The character set of the current character.".to_string();
            *color = "white";
        } else if prompt.starts_with("sets=") {
            if prompt[5..].len() == 0 {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be a non-empty string.";
                *color = "red";
            } else if prompt[5..].len() != prompt[5..].chars().count() {
                *hide = true;
                *message = "ERROR - Invalid value: '".to_string();
                *message += &prompt[5..];
                *message += "'. The value must be a string containing only ASCII characters.";
                *color = "red";
            } else {
                cache.sets.insert(cache.index.to_string(), prompt[5..].to_string());
                *hide = false;
                *message = "".to_string();
            }
        }
    } else if prompt == "help" {
        *hide = true;
        *message = "Type 'option=<value>' to set a value for an option.\nType the name of an option to get more information about it.\nType 'exit', 'quit', 'q', 'e', 'Exit', 'Quit', 'Q' or 'E' to exit the program.\nPush 'Enter' to start the brute force algorithm.".to_string();
        *color = "white";
    } else {
        if prompt.contains("=") {
            *message = "ERROR - Invalid option: '".to_string();
            *message = prompt[0..prompt.find("=").unwrap()].to_string();
            *message += "'. Type 'help' for more information.";
        } else {
            *message = "ERROR - Invalid command: '".to_string();
            *message += &prompt;
            *message += "'. Type 'help' for more information.";
        }
        *hide = true;
        *color = "red";
    }

    if prompt == "" || prompt == "exit" || prompt == "quit" || prompt == "q" || prompt == "e" || prompt == "Exit" || prompt == "Quit" || prompt == "Q" || prompt ==" E" {
        if prompt == "" {
            if cache.target == "zip" && (!Path::new(&cache.name).exists() || !cache.name.ends_with(".zip")) {
                *hide = true;
                *message = "ERROR - Invalid name value: '".to_string();
                *message += &cache.name;
                *message += "'. The value must be a valid path to a zip file.";
                *color = "red";
            } else {
                let cache_json = to_string_pretty(cache).expect("Failed to serialize cache");
                write(file_path, cache_json).expect("Failed to write cache");
                println!("\n{}\n", "START - Brute force algorithm".green());
            }
        } else {
            println!("\n{}\n", "END - User exit".red());
            exit(0);
        }
    } else { menu(file_path, cache, hide, message, color, prompt) }
}

// Brute force algorithm
fn cracker(cache: &mut Cache) { // file_path: &str
    const GOAL: &str = "Abizu1"; //* The password to crack (for testing purposes).
    let mut attempted: usize = 1;
    let mut skipped: usize = 1;
    let mut depth: usize = cache.min;
    let mut counters: [usize; 16] = [0; 16];
    for i in 1..=16 { counters[i-1] = cache.sets[i.to_string().as_str()].len(); }
    let mut result = false;

    //* We declare these variables outside the loop to avoid creating them every iteration.
    let mut password;
    let mut valid;
    let mut same;
    let mut seq;

    while depth <= cache.max {
        //* We use `.as_bytes()` to access characters in strings because this method doesn't create an iterator (like `.chars()`) which is more efficient.
        // TODO: Use `[0..1]` to extract the first character (as a string) instead of `as_bytes()[0]` where possible because they have the same time complexity (O(1)) but the former is more readable. Note that there is no equivalent to `as_bytes().last()` for string slices with the same time complexity (O(n)).
        password = cache.sets["1"][0..1].to_string();
        valid = true;
        seq = 0;

        if depth >= 2 {
            for k in 2..=depth {
                same = 0;
                if cache.case {
                    // Sequence check
                    if password.as_bytes().last().unwrap() == &cache.sets[k.to_string().as_str()].as_bytes()[0] {
                        seq += 1;
                        if seq > cache.seq { valid = false; break }
                    } else { seq = 0 }
                    
                    // Same check
                    //? Is it possible to perform this check without creating a second loop?
                    for &b in password.as_bytes() {
                        if b == cache.sets[k.to_string().as_str()].as_bytes()[0] {
                            same += 1;
                            if same == cache.same { valid = false; break }
                        }
                    }
                } else {
                    // Sequence check
                    if (*password.as_bytes().last().unwrap() as char).to_uppercase().next().unwrap() == (cache.sets[k.to_string().as_str()].as_bytes()[0] as char).to_uppercase().next().unwrap() {
                        seq += 1;
                        if seq > cache.seq { valid = false; break }
                    } else { seq = 0 }

                    // Same check
                    //? Is it possible to perform this check without creating a second loop?
                    for &b in password.as_bytes() {
                        if (b as char).to_uppercase().next().unwrap() == (cache.sets[k.to_string().as_str()].as_bytes()[0] as char).to_uppercase().next().unwrap() {
                            same += 1;
                            if same == cache.same { valid = false; break }
                        }
                    }
                }
                if valid { password += &cache.sets[k.to_string().as_str()][0..1] }
            }
        }

        if valid {
            if GOAL == password {
                let mut s = String::from("(");
                s += &attempted.to_string().as_str();
                s += ") (";
                s += &skipped.to_string().as_str();
                s += ") [";
                s += &password;
                s += "] --> SUCCESS";
                println!("{}", s.green());
                result = true;
                break;
            } else {
                if attempted % cache.freq == 0 { println!("({}) ({}) [{}] --> FAILURE", attempted, skipped, password) }
                attempted += 1;
            }
        } else {
            if skipped % cache.freq == 0 { println!("({}) ({}) [{}] --> SKIPPED", attempted, skipped, "#".to_string().repeat(depth)) }
            skipped += 1;
        }
    
        for i in (1..=depth).rev() {
            if cache.sets[i.to_string().as_str()].len() > 1 {
                cache.sets.insert(i.to_string(), cache.sets[i.to_string().as_str()][1..].to_string() + &cache.sets[i.to_string().as_str()][0..1]);
            }
            counters[i-1] -= 1;
            if i != 1 && counters[i-1] == 0 { counters[i-1] = cache.sets[i.to_string().as_str()].len() }
            else { break }
        }
        if counters[0] == 0 { depth += 1; counters[0] = cache.sets["1"].len() }
    }

    if result { println!("\n{}\n", "END - Password found".red()) }
    else { println!("\n{}\n", "END - No password found".red()) }
}


fn main() {
    // TODO: Make a multi-threaded version of the program.
    // TODO: Find a way to deserialize the cache.sets into a HashMap<usize, &str> instead of HashMap<&str, &str>. That way we can use the `k` variable as a key without having to convert it to a string (`.to_string().as_str()`) which is an operation with an O(n) time complexity.
    //? For some strange reason, this version of the program and it's PowerShell equivalent don't produce the same final number of attempts and skips.
    // Opens the cache file, read it into a string and deserialize it into a struct.
    const FILE_PATH: &str = "cache.json";
    let mut cache_file = File::open(FILE_PATH).expect("Could not open cache file");
    let mut cache_string = String::new(); // Stays alive for the entire program.
    cache_file.read_to_string(&mut cache_string).expect("Could not read cache file");
    let mut cache: Cache = serde_json::from_str(&cache_string).expect("Error while reading json");

    menu(&FILE_PATH, &mut cache, &mut false, &mut "".to_string(), &mut "", &mut "".to_string());
    cracker(&mut cache);
}
