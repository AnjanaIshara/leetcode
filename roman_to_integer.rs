use std::collections::HashMap;

fn roman_to_int(s: &str)-> i32 {
    let mut roman_map = HashMap::new();
    roman_map.insert('I', 1);
    roman_map.insert('V', 5);
    roman_map.insert('X', 10);
    roman_map.insert('L', 50);
    roman_map.insert('C', 100);
    roman_map.insert('D', 500);
    roman_map.insert('M', 1000);

    let mut result = 0; 
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for i in 0..len {
        let current_value = roman_map[&chars[i]];
        if i < len -1 && current_value < roman_map[&chars[i + 1]] {
            result -= current_value;
        }
        else {
            result += current_value;
        }
    }
    result
}

fn main() {
    let roman = "MCMXCIV";
    println!("The Integer value of {} is {} ", roman, roman_to_int(roman));
}