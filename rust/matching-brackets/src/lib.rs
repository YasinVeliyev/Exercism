pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = vec![];
    // for b in string.chars() {
    //     let check = if b == '[' {
    //         brackets.push(']');
    //         true
    //     } else if b == ']' {
    //         brackets.pop() == Some(']')
    //     } else if b == '(' {
    //         brackets.push(')');
    //         true
    //     } else if b == ')' {
    //         brackets.pop() == Some(')')
    //     } else if b == '{' {
    //         brackets.push('}');
    //         true
    //     } else if b == '}' {
    //         brackets.pop() == Some('}')
    //     } else {
    //         true
    //     };
    //     if !check {
    //         return false;
    //     }

    // let check = match b {
    //     '[' => {
    //         brackets.push(']');
    //         true
    //     }
    //     ']' => brackets.pop() == Some(']'),
    //     '(' => {
    //         brackets.push(')');
    //         true
    //     }
    //     ')' => brackets.pop() == Some(')'),
    //     '{' => {
    //         brackets.push('}');
    //         true
    //     }
    //     '}' => brackets.pop() == Some('}'),
    //     _ => true,
    // };
    // if !check {
    //     return check;
    // }
    //}
    for b in string.chars() {
        match b {
            '(' | '{' | '[' => brackets.push(b),
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }

    brackets.is_empty()
}
