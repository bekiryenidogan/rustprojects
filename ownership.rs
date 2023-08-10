
fn concantenate_strings (str1: &str, str2: &str) -> String {
let mut result = String::new();
result.push_str(str1);
result.push_str(str2);
result
}

fn main() {
    let string1 = "Hello, ";
    let string2 = "Patika!";

    let concantenate_string = concantenate_strings(string1, string2);

    println! ( "{}", concantenate_string);
}