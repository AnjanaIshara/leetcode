use std::collections::HashMap;

fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty()
    {
        return String::new();
    }
    
    let mut prefix = strs[0].to_string();
    for s in strs.iter().skip(1) {
        while ! s.starts_with(&prefix) {
            prefix.pop();
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix
    
}

fn main() {
    let roman = "MCMXCIV";
    println!("The Integer value of {} is {} ", roman, roman_to_int(roman));
}