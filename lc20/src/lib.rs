use std::collections::VecDeque;

pub fn is_valid(s: String) -> bool {
    let mut currently_open_brackets = VecDeque::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => currently_open_brackets.push_back(c),
            ')' | ']' | '}' => {
                let wanted = match c {
                    ')' => '(',
                    ']' => '[',
                    '}' => '{',
                    _ => panic!(),
                };
                if let Some(open_bracket) = currently_open_brackets.pop_back() {
                    if open_bracket != wanted {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => panic!("invalid input."),
        }
    }

    currently_open_brackets.is_empty()
}

#[test]
fn example_1() {
    let s = "()";
    assert_eq!(is_valid(s.into()), true);
}

#[test]
fn example_2() {
    let s = "()[]{}";
    assert_eq!(is_valid(s.into()), true);
}

#[test]
fn example_3() {
    let s = "(]";
    assert_eq!(is_valid(s.into()), false);
}
