// Is Unique: implement an algorithm to determine if a string has all
// unique characters. What if you cannot use additional data structures?

pub fn run() {
    let word = "hello"; // duplicate ll in the word
    is_unique_chars(word);
}

fn is_unique_chars(my_string: &str) -> bool {
    // for each character in the string, compare that character
    // to the other characters in the string; if there is a repeat
    // character, then return false;
    // else return true
    let mut pos = 0;
    for c in my_string.chars() { // .chars turns &str into an iterator
        pos += 1;
        let mut current_pos = 0;
        // need to
        for sc in my_string.chars() {
            current_pos += 1;
            if (current_pos > pos) {
                // compare the characters to each other
                println!("{}", sc);
                if c == sc {
                    return false;
                }
            }
        }
    }
    return true;
}
