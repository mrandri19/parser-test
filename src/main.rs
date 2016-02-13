use std::env;

fn main() {
    let input = env::args().nth(1).expect("No argument");

    let parsed = parse(input);
    print!("{:?}\n", parsed);
}

fn parse(input: String) -> Result<String, String> {
    let input_str: Vec<char> = input.replace(" ", "").chars().collect();

    let mut index = 0;

    // root : number expr
    if input_str[index].is_digit(10) {
        print!("is_is_digit #0: ");
        inc_index(&mut index, &input_str);
    } else {
        let err_msg = format!("char: {} should be a digit", index);
        return Err(err_msg);
    }

    let result = expr(&mut index, &input_str);
    if result.is_err() {
        return result;
    }

    Ok("da".to_string())
}


// expr : operator number [expr]
fn expr(index: &mut usize, input_str: &Vec<char>) -> Result<String, String> {
    let backtrack = *index;

    if is_operator(&(*input_str)[*index]) {
        print!("is_operator: ");
        if !inc_index(index, input_str) {
            return Err("failed to increase the index".to_string());
        }
    } else {
        return Err("The first char of an expr should be an operator".to_string());
    }

    if (*input_str)[*index].is_digit(10) {
        print!("is_digit: : ");
        if !inc_index(index, input_str) {
            return Err("failed to increase the index".to_string());
        }
    } else {
        return Err("The second char of an expr should be a digit".to_string());
    }

    // Check if we finished
    print!("Did we finish?\n");

    let index_plus_one = &(*index + 1);
    if finished(index_plus_one, input_str) {
        return Ok("finished".to_string());
    } else {
        return expr(index, input_str);
    }
}

// Operator /[\+\-\*\/]/
fn is_operator(ch: &char) -> bool {
    match *ch {
        '+' | '-' | '*' | '/' => true,
        _ => false,
    }
}

fn inc_index(index: &mut usize, input_str: &Vec<char>) -> bool {
    *index += 1;
    if finished(index, input_str) {
        return false;
    }
    print!("{}\n", *index);
    true
}

fn finished(index: &usize, input_str: &Vec<char>) -> bool {
    // If it were an equal condition we could be after the penultimate char but
    // not finished yet
    if (*input_str).len() < (*index) {
        return true;
    }
    false
}
