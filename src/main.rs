use std::env;

fn main() {
    let input = env::args().nth(1).expect("No argument");

    let parsed = parse(&input);
    println!("{:?}", parsed);
}

fn parse<'a>(input: &'a str) -> Result<f64, String> {
    let mut counter: f64;

    if input.len() == 0 {
        return Err("Empty input".to_string());
    }

    let input_str: Vec<char> = input.replace(" ", "").chars().collect();
    let mut index = 0;

    // root : number expr
    if input_str[index].is_digit(10) {
        print!("is_is_digit #0: ");
        counter = input_str[index].to_digit(10).unwrap() as f64;
        inc_index(&mut index, &input_str);
    } else {
        let err_msg = format!("char: {} should be a digit", index);
        return Err(err_msg);
    }

    if finished(&(index + 1), &input_str) {
        return Ok(counter);
    }
    let result = expr(&mut index, &input_str, &mut counter);
    if result.is_err() {
        return result;
    }

    Ok(result.unwrap())
}


// expr : operator number [expr]
fn expr(index: &mut usize, input_str: &Vec<char>, counter: &mut f64) -> Result<f64, String> {
    let backtrack = *index;
    let current_operator: char;

    if is_operator(&(*input_str)[*index]) {
        print!("is_operator: ");

        current_operator = (*input_str)[*index];

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
        let digit = (*input_str)[*index].to_digit(10).unwrap() as f64;

        match current_operator {
            '+' => *counter += digit,
            '-' => *counter -= digit,
            '*' => *counter *= digit,
            '/' => {
                if digit != 0.0 {
                    *counter /= digit;
                } else {
                    return Err("Can't divide by 0".to_string());
                }
            }
            _ => (),
        }

        if !inc_index(index, input_str) {
            return Err("failed to increase the index".to_string());
        }
    } else {
        return Err("The second char of an expr should be a digit".to_string());
    }

    // Check if we finished
    println!("Did we finish?");

    // index_plus_one is basically asking wheter there is more to parse
    if finished(&(*index + 1), input_str) {
        println!("yes!");
        return Ok(*counter);
    } else {
        println!("no!");
        return expr(index, input_str, counter);
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
    println!("{}", *index);
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
// TODO: implement negative_numbers
// TODO: implement operation order

#[test]
fn empty_input() {
    assert_eq!(Err("Empty input".to_string()), parse(""));
}

#[test]
fn single_digit() {
    assert_eq!(Ok(1.0), parse("1"));
}

#[test]
fn single_not_digit() {
    assert_eq!(Err("char: 0 should be a digit".to_string()), parse("d"));
}

#[test]
fn expr_first_not_operator() {
    assert_eq!(Err("The first char of an expr should be an operator".to_string()),
               parse("1d"));
}

#[test]
fn incomplete_expr() {
    assert_eq!(Err("Digit expected at position: 3".to_string()),
               parse("1+"));
}

#[test]
fn expr_second_not_digit() {
    assert_eq!(Err("The second char of an expr should be a digit".to_string()),
               parse("1+d"));
}

#[test]
fn multiple_expr() {
    assert_eq!(Ok(7.0), parse("1+1+1+1+1+1+1"));
}

#[test]
fn negative_number() {
    assert_eq!(Ok(-16.0), parse("0-8-8"));

}

#[test]
fn divide_by_zero() {
    assert_eq!(Err("Can't divide by 0".to_string()), parse("1/0"));
}
