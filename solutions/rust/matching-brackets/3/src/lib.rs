pub fn brackets_are_balanced(string: &str) -> bool {
    use std::collections::HashMap;
    let mut stack: Vec<char> = Vec::new();

    let mut pairs = HashMap::new();
    pairs.insert('}', '{');
    pairs.insert(']', '[');
    pairs.insert(')', '(');

    for x in string.chars() {
        // if it's an opener, push
        if pairs.values().any(|&open| open == x) {
            stack.push(x);
        }
        // if it's a closer, check the top
        else if let Some(&expected) = pairs.get(&x) {
            if stack.last() == Some(&expected) {
                stack.pop();
            } else {
                return false;
            }
        }
        // ignore other characters
    }

    stack.is_empty()
}
