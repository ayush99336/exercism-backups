pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let stringer: Vec<char> = string.chars().collect();
    for x in stringer {
        if x == '{' {
            stack.push(x);
        } else if x == '[' {
            stack.push(x);
        } else if x == '(' {
            stack.push(x);
        }
        if x == '}' {
            if stack.last() == Some(&'{') {
                stack.pop();
            } else {
                return false;
            }
        } else if x == ']' {
            if stack.last() == Some(&'[') {
                stack.pop();
            } else {
                return false;
            }
        } else if x == ')' {
            if stack.last() == Some(&'(') {
                stack.pop();
            } else {
                return false;
            }
        }
    }
    if stack.is_empty() == true {
        return true;
    }
    return false;
}
