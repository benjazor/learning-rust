fn main() {
    let my_string = "     Coucou :)  ";
    println!(
        "|{}|\ntrimming spaces...\n|{}|",
        my_string,
        trim_spaces(my_string)
    );
}

fn trim_spaces(s: &str) -> &str {
    let mut start = 0;
    loop {
        if s.as_bytes()[start] as char == ' ' {
            start += 1;
        } else {
            break;
        }
    }

    let mut end = s.len() - 1;
    loop {
        if s.as_bytes()[end] as char == ' ' {
            end -= 1;
        } else {
            break;
        }
    }

    &s[start..end + 1]
}
