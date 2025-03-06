fn is_valid_brackets(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let bracket_map = [('(', ')'), ('[', ']'), ('{', '}')];

    for c in s.chars() {
        if bracket_map.iter().any(|&(open, _)| open == c) {
            stack.push(c);
        } else if bracket_map.iter().any(|&(_, close)| close == c) {
            match stack.pop() {
                Some(open) => {
                    if !bracket_map.iter().any(|&(o, cl)| o == open && cl == c) {
                        return false;
                    }
                }
                None => return false,
            }
        }
    }

    stack.is_empty()
}


fn clean_phone_number(number: &str) -> Option<String> {
    let mut cleaned_number: String = number
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();

    if cleaned_number.len() == 12 && cleaned_number.starts_with("38") {
        cleaned_number = cleaned_number[2..].to_string();
    } else if cleaned_number.len() == 11 && cleaned_number.starts_with("8") {
        cleaned_number = cleaned_number[1..].to_string();
    }


    if cleaned_number.len() == 10 {
        Some(cleaned_number)
    } else {
        None
    }
}

fn main() {
    println!("([]{{}})[] is valid: {}", is_valid_brackets("([]{})[]"));
    println!("([]] is valid: {}", is_valid_brackets("([]]"));
    println!("(( is valid: {}", is_valid_brackets("(("));
    println!(") is valid: {}", is_valid_brackets(")"));
    
    let phone_numbers = vec![
        "+3 (050)-995-0253",
        "050-995-0253",
        "3 050 995 0253",
        "050.995.0253",
        "380509950253",
        "80509950253",
        "0631111111",
        "12345",
    ];

    for number in phone_numbers {
        match clean_phone_number(number) {
            Some(cleaned) => println!("{} -> {}", number, cleaned),
            None => println!("{} -> Invalid number", number),
        }
    }
}