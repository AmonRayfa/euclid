use colored::*; // Used to color the output.


// Brute force algorithm
pub fn cracker(cache: &mut crate::cache::Cache) { // file_path: &str
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
