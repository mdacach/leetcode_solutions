pub fn max_value(n: String, x: i32) -> String {
    let x_as_char = char::from_digit(x as u32, 10).unwrap();
    let len = n.len();
    let insert_at = match n.chars().next().unwrap() {
        '-' => n.chars().position(|digit| digit > x_as_char).unwrap_or(len),
        _ => n.chars().position(|digit| digit < x_as_char).unwrap_or(len),
    };

    let mut answer = n;
    answer.insert(insert_at, x_as_char);

    answer
}

#[test]
fn example_1() {
    let n = String::from("99");
    let x = 9;

    assert_eq!(max_value(n, x), "999");
}

#[test]
fn example_2() {
    let n = String::from("-13");
    let x = 2;

    assert_eq!(max_value(n, x), "-123");
}
