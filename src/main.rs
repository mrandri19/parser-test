use std::env;

fn main() {
    let input = env::args().nth(1).expect("No argument");

    let parsed = parse(input);
    print!("{:?}\n", parsed);
}

fn parse(input: String) -> Result<String, String> {
    let inputStr: Vec<char> = input.replace(" ", "").chars().collect();

    let mut resultStr: String;

    let mut index = 0;

    // root : number expr
    if inputStr[index].is_digit(10) {
        incIndex(&mut index);
    } else {
        let err_msg = format!("char: {} should be a digit", index);
        return Err(err_msg);
    }

    expr(&mut index, &inputStr);

    // print!("{}\n", index);

    Ok("da".to_string())
}

fn expr(index: &mut usize, inputStr: &Vec<char>) -> bool {
    let backtrack = *index;

    if is_operator(&(*inputStr)[*index]) {
        incIndex(index);
    } else {
        return false;
    }

    if (*inputStr)[*index].is_digit(10) {
        incIndex(index);
    } else {
        return false;
    }

    // Check if we finished
    if (*inputStr).len() == (*index) {
        return true;
    } else {
        return expr(index, inputStr);
    }
}

fn is_operator(ch: &char) -> bool {
    match *ch {
        '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn incIndex(i: &mut usize) {
    *i += 1;
    print!("{}\n", *i);
}
