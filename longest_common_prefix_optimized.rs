fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty()
    {
        return String::new();
    }
    
    let mut prefix = &strs[0][..];
    for s in strs.iter().skip(1) {
        while ! s.starts_with(&prefix) {
            prefix = &prefix[..prefix.len()-1];
            if prefix.is_empty() {
                return String::new();
            }
        }
    }
    prefix.to_string()
    
}

fn main() {
    let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    println!("Longest common prefix {} ", longest_common_prefix(strs));
}