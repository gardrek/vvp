
fn main() {
    let _addr = AddressSpace::new();
    for i in 0..27 {
        println!("{}: {}", i, digit_to_heptavintimal(i));
    };
    for i in 0..32 {
        println!("{}: {}", i, digit_to_crockford(i));
    };
}

fn heptavintimal_char_to_base27(c: char) -> char {
    match c {
        'k' => 'i',
        'm' => 'j',
        'n' => 'k',
        'p' => 'l',
        'r' => 'm',
        't' => 'n',
        'v' => 'o',
        'x' => 'p',
        'z' => 'q',
        _ => c,
    }
}

fn digit_to_heptavintimal(digit: u32) -> char {
    let mut c = std::char::from_digit(digit, 27).unwrap();
    c.make_ascii_lowercase();
    match c {
        'i' => 'k',
        'j' => 'm',
        'k' => 'n',
        'l' => 'p',
        'm' => 'r',
        'n' => 't',
        'o' => 'v',
        'p' => 'x',
        'q' => 'z',
        _ => c,
    }
}


fn digit_to_crockford(digit: u32) -> char {
    let mut c = std::char::from_digit(digit, 32).unwrap();
    c.make_ascii_lowercase();
    match c {
        'i' => 'j',
        'j' => 'k',
        'k' => 'm',
        'l' => 'n',
        'm' => 'p',
        'n' => 'q',
        'o' => 'r',
        'p' => 's',
        'q' => 't',
        'r' => 'v',
        's' => 'w',
        't' => 'x',
        'u' => 'y',
        'v' => 'z',
        _ => c,
    }
}
