use std::path::Path; // Used to check if a file exists.
use std::fs::write; // Used to open and write to a file.
use std::io::{ self, Write };
use std::process::exit; // Used to exit the program.

use serde_json; // Used to serialize the cache struct into a JSON string.
use colored::*; // Used to color the output.


// Menu function
pub fn menu(file_path: &str, cache: &mut crate::cache::Cache, hide: &mut bool, message: &mut String, color: &mut &str, prompt: &mut String) {
    
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
                let cache_json = serde_json::to_string_pretty(cache).expect("Failed to serialize cache");
                write(file_path, cache_json).expect("Failed to write cache");
                println!("\n{}\n", "START - Brute force algorithm".green());
            }
        } else {
            println!("\n{}\n", "END - User exit".red());
            exit(0);
        }
    } else { menu(file_path, cache, hide, message, color, prompt) }
}
