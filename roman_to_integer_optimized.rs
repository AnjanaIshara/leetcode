use std::collections::HashMap;

fn roman_to_int(s: &str)-> i32 {
    let mut result = 0;
    let mut prev_value = 0;
    for c in s.chars().rev() {
        let current_value = match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000, 
            _ => 0,
        };
        if current_value < prev_value {
            result -= current_value;
        }
        else {
            result += current_value;
        }
        prev_value = current_value;
    }
    result
}

fn main() {
    let roman = "MCMXCIV";
    println!("The Integer value of {} is {} ", roman, roman_to_int(roman));
}