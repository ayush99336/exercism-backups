pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for x in string.chars() {
        match x {
            '{' | '[' | '(' => stack.push(x),
            '}' => {
                if stack.last() == Some(&'{') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if stack.last() == Some(&'[') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            ')' => {
                if stack.last() == Some(&'(') {
                    stack.pop();
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}
