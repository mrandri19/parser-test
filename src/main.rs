use std::env;

fn main() {
    let input = env::args().nth(1).expect("No argument");

    let parsed = parse(input);
    print!("{:?}\n", parsed);
}

fn parse(input: String) -> Result<String, String> {
    if input.len() == 0 {
        return Err("Empty input".to_string());
    }

    let input_str: Vec<char> = input.replace(" ", "").chars().collect();
    let mut output = String::new();

    let mut index = 0;

    // root : number expr
    if input_str[index].is_digit(10) {
        print!("is_is_digit #0: ");
        output.push(input_str[index]);
        inc_index(&mut index, &input_str);
    } else {
        let err_msg = format!("char: {} should be a digit", index);
        return Err(err_msg);
    }

    if finished(&(index + 1), &input_str) {
        return Ok(output);
    }
    let result = expr(&mut index, &input_str, output);
    if result.is_err() {
        return result;
    }

    Ok(result.unwrap())
}


// expr : operator number [expr]
fn expr(index: &mut usize, input_str: &Vec<char>, mut output: String) -> Result<String, String> {
    let backtrack = *index;

    if is_operator(&(*input_str)[*index]) {
        print!("is_operator: ");
        output.push((*input_str)[*index]);
        if !inc_index(index, input_str) {
            return Err("failed to increase the index".to_string());
        }
    } else {
        return Err("The first char of an expr should be an operator".to_string());
    }

    if finished(&(*index + 1), input_str) {
        let err_msg = format!("Digit expected at position: {}", *index + 1);
        return Err(err_msg);
    }
    if (*input_str)[*index].is_digit(10) {
        print!("is_digit: ");
        output.push((*input_str)[*index]);
        if !inc_index(index, input_str) {
            return Err("failed to increase the index".to_string());
        }
    } else {
        return Err("The second char of an expr should be a digit".to_string());
    }

    // Check if we finished
    print!("Did we finish?\n");

    // index_plus_one is basically asking wheter there is more to parse
    if finished(&(*index + 1), input_str) {
        println!("yes!");
        return Ok(output);
    } else {
        println!("no!");
        return expr(index, input_str, output);
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
    print!("{} -> ", *index);
    *index += 1;
    print!("{}\n", *index);
    println!("calling finished() with args {}, {}",
             *index,
             (*input_str).len());
    if finished(index, input_str) {
        println!("We finished, returning false");
        return false;
    }
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
// TODO: there_is_more

#[test]
fn empty_input() {
    assert_eq!(Err("Empty input".to_string()), parse("".to_string()));
}

#[test]
fn single_digit() {
    assert_eq!(Ok("1".to_string()), parse("1".to_string()));
}
#[test]
fn single_not_digit() {
    assert_eq!(Err("char: 0 should be a digit".to_string()),
               parse("d".to_string()));
}
#[test]
fn expr_first_not_operator() {
    assert_eq!(Err("The first char of an expr should be an operator".to_string()),
               parse("1d".to_string()));
}

#[test]
fn incomplete_expr() {
    assert_eq!(Err("Digit expected at position: 3".to_string()),
               parse("1+".to_string()));
}

#[test]
fn expr_second_not_digit() {
    assert_eq!(Err("The second char of an expr should be a digit".to_string()),
               parse("1+d".to_string()));
}
