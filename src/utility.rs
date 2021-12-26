pub fn is_peelable(expr: &str) -> bool {
    // peelable := contains no brackets, or contains brackets that are nested properly such that they encapsulate an interior that is also peelable
    // checks if all lines in the code have matching '(' and ')'
    let left_c : i8 = expr.matches('(').count() as i8; // counts all '('
    let right_c : i8 = expr.matches(')').count() as i8; // counts all ')'
    // preliminary condition checks if num of '(' and num of ')' are equal
    if left_c != right_c{
        return false // short circuit
    } else {
    }
    // preliminary condition checks if they are both 0 (assuming first condition passes)
    if left_c == 0{
        return true // short circuit
    } else {
    }
    // if the first bracket occurence is a ')' it's automatically invalid
    for ch in expr.chars() {
        if ch == ')' {
            return false // short circuit
        }else if ch == '(' {
            break;
        }else {
        };
    }
    // if the last bracket occurence is a '(' it's automatically invalid
    for ch in expr.chars().rev() {
        if ch == '(' {
            return false // short circuit
        }else if ch == ')' {
            break;
        }else {
        };
    }
    return true // all tests passed therefore line is peelable
}

// pub fn strip_trailing_newline(input: &str) -> &str {
//     input
//         .strip_suffix("\r\n")
//         .or(input.strip_suffix("\n"))
//         .unwrap_or(input)
// }
