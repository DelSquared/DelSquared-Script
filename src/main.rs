use std::env;
use std::fs;
use std::fmt;
use std::collections::HashMap;
use std::time::Instant;

union Val {
    i: i32,
    f: f32,
}

struct Num {
    typ: char, // Hopefully can get away with just i, f, etc. Will swap to String if one char is insufficient
    val: Val,
}

impl fmt::Debug for Num {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *&self.typ == 'i'{
            unsafe{ f.write_fmt(format_args!("{}", &self.val.i))}
        } else if *&self.typ == 'f'{
            unsafe{ f.write_fmt(format_args!("{}", &self.val.f))}
        } else {
            f.write_fmt(format_args!("{}?", &self.typ)) //weird type wtf?
        }
    }
}

fn is_peelable(expr: &str) -> bool {
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

fn main() {
    // commandline args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("\n===== In file: =====\n\n{}\n", filename);

    // read file specified by commandline args
    let now = Instant::now();
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let elapsed = now.elapsed();
    println!("\n===== With text: =====\n\n{}\n", content);
    println!("Elapsed: {:.2?}\n", elapsed);

    // parse lines and strip empty (i.e. useless) lines
    let now = Instant::now();
    let mut content: Vec<&str> = content.split("\n").collect();
    while let Some(ln) = content.iter().position(|x| *x == "\r") {
        content.remove(ln);
    }
    while let Some(ln) = content.iter().position(|x| *x == "") {
        content.remove(ln);
    }
    let bracket_check: bool = !content.iter().any(|ln| !is_peelable(ln));
    let elapsed = now.elapsed();
    println!("\n===== Split text: =====\n\n{:?}\n", content);
    // Checks if the script has the correct use of brackets (in the future this will cause interpreter to panic!)
    println!("Correct use of brackets: {}\n",bracket_check);
    println!("Elapsed: {:.2?}\n", elapsed);

    // define table for numerical variables
    let now = Instant::now();
    let mut num_table: HashMap<&str, Num> = HashMap::new();

    // example variable initialisation (remove later)
    println!("\n===== Var table tests: =====\n\n");
    println!("{:#?}\n", num_table);
    num_table.insert("test_var", Num{typ:'i', val:Val{i:100}});
    println!("{:#?}\n", num_table);
    num_table.insert("test_var2", Num{typ:'f', val:Val{f:10.1}});
    println!("{:#?}\n", num_table);
    num_table.remove("test_var");
    println!("{:#?}\n", num_table);
    num_table.remove("test_var2");
    println!("{:#?}\n", num_table);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);

    // define table for boolean variables
    let now = Instant::now();
    let mut bool_table: HashMap<&str, bool> = HashMap::new();

    // example variable initialisation (remove later)
    println!("\n===== Var table tests: =====\n\n");
    println!("{:?}\n", bool_table);
    bool_table.insert("test_var", true);
    println!("{:?}\n", bool_table);
    bool_table.insert("test_var2", false);
    println!("{:?}\n", bool_table);
    bool_table.remove("test_var");
    println!("{:?}\n", bool_table);
    bool_table.remove("test_var2");
    println!("{:?}\n", bool_table);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}\n", elapsed);

    // define regex patterns for parsing... Might not keep this idk
    let now = Instant::now();
    let mut parse_patterns: Vec<&str> = Vec::new();
    parse_patterns.push(r"(\w*)\((.*)\)"); //function
    let parse_patterns: Vec<&str> = parse_patterns;
    let elapsed = now.elapsed();
    println!("\n===== Keywords: =====\n\n{:?}\n", parse_patterns);
    println!("Elapsed: {:.2?}\n", elapsed);

    // define language keywords
    let now = Instant::now();
    let mut kwds: Vec<&str> = Vec::new();
    kwds.push("if");
    kwds.push("else");
    kwds.push("end");
    kwds.push("label");
    kwds.push("goto");
    let elapsed = now.elapsed();
    let kwds: Vec<&str> = kwds;
    println!("\n===== Keywords: =====\n\n{:?}\n", kwds);
    println!("Elapsed: {:.2?}\n", elapsed);

    // define language binary numerical operators
    let now = Instant::now();
    let mut bin_num_ops: HashMap<String, fn(f32,f32)->f32> = HashMap::new();
    bin_num_ops.insert("+".to_string(), |x, y| x + y);
    bin_num_ops.insert("-".to_string(), |x, y| x - y);
    bin_num_ops.insert("*".to_string(), |x, y| x * y);
    bin_num_ops.insert("/".to_string(), |x, y| x / y);
    let bin_num_ops = bin_num_ops;
    let elapsed = now.elapsed();
    println!("\n===== Binary Numerical Operators: =====\n\n{:?}\n", bin_num_ops.keys().cloned().collect::<Vec<String>>());
    println!("Elapsed: {:.2?}\n", elapsed);

    // define language logical operators
    let now = Instant::now();
    let mut bin_log_ops: HashMap<String, fn(bool,bool)->bool> = HashMap::new();
    bin_log_ops.insert("&".to_string(), |x,  y| x & y);
    bin_log_ops.insert("|".to_string(), |x,  y| x | y);
    bin_log_ops.insert("^".to_string(), |x,  y| x ^ y);
    bin_log_ops.insert("!".to_string(), |x, _y| !x );
    let bin_log_ops = bin_log_ops;
    let elapsed = now.elapsed();
    println!("\n===== Logical Operators: =====\n\n{:?}\n", bin_log_ops.keys().cloned().collect::<Vec<String>>());
    println!("Elapsed: {:.2?}\n", elapsed);

    // define language comparative operators
    let now = Instant::now();
    let mut bin_comp_ops: HashMap<String, fn(f32,f32)->bool> = HashMap::new();
    bin_comp_ops.insert("<=".to_string(), |x, y| x <= y);
    bin_comp_ops.insert(">=".to_string(), |x, y| x >= y);
    bin_comp_ops.insert("==".to_string(), |x, y| x == y);
    bin_comp_ops.insert("!=".to_string(), |x, y| x != y);
    bin_comp_ops.insert("<".to_string(),  |x, y| x <  y);
    bin_comp_ops.insert(">".to_string(),  |x, y| x >  y);
    let bin_comp_ops = bin_comp_ops;
    let elapsed = now.elapsed();
    println!("\n===== Binary Comparative Operators: =====\n\n{:?}\n", bin_comp_ops.keys().cloned().collect::<Vec<String>>());
    println!("Elapsed: {:.2?}\n", elapsed);

    // define language control operators
    let now = Instant::now();
    let mut bin_ctrl_ops: HashMap<String, i32> = HashMap::new();
    bin_ctrl_ops.insert("=".to_string(), 0); // initialise
    bin_ctrl_ops.insert("~".to_string(), 1); // destroy
    let bin_ctrl_ops = bin_ctrl_ops;
    let elapsed = now.elapsed();
    println!("\n===== Control Operators: =====\n\n{:?}\n", bin_ctrl_ops.keys().cloned().collect::<Vec<String>>());
    println!("Elapsed: {:.2?}\n", elapsed);

}
