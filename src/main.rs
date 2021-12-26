use std::{env,fs};
use std::collections::HashMap;
use std::time::Instant;

mod utility;
//use utility::{is_peelable,strip_trailing_newline};
use utility::is_peelable;
mod stack_manipulation;
use stack_manipulation::{sum,prod,eval_rpn};
mod typing;
//use typing::Num;

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

    //let content = "RPN : 2 2 + 1 - = first_eval\r\nRPN : 2 2 2 2 $ 5 5 £ 10 - = second_eval";

    // parse lines and strip empty (i.e. useless) lines
    let now = Instant::now();
    let mut content: Vec<&str> = content.split("\r\n").collect();
    while let Some(ln) = content.iter().position(|x| *x == "\r") {
        content.remove(ln);
    }
    while let Some(ln) = content.iter().position(|x| *x == "") {
        content.remove(ln);
    }
    let bracket_check: bool = !content.iter().any(|ln| !is_peelable(ln));
    // content.iter().map(|x| strip_trailing_newline(x))
    let elapsed = now.elapsed();
    println!("\n===== Split text: =====\n\n{:?}\n", content);
    // Checks if the script has the correct use of brackets (in the future this will cause interpreter to panic!)
    println!("Correct use of brackets: {}\n",bracket_check);
    println!("Elapsed: {:.2?}\n", elapsed);

    // define table for float variables
    let now = Instant::now();
    let mut num_table: HashMap<&str, f32> = HashMap::new();

    // example variable initialisation (remove later)
    println!("\n===== Var table tests: =====\n\n");
    println!("{:?}\n", num_table);
    num_table.insert("test_var", 100.0);
    println!("{:?}\n", num_table);
    num_table.insert("test_var2", 10.1);
    println!("{:?}\n", num_table);
    num_table.remove("test_var");
    println!("{:?}\n", num_table);
    num_table.remove("test_var2");
    println!("{:?}\n", num_table);
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
    kwds.push("goto"); // primitive way to eventually handle looping and functions when starting off
    kwds.push("RPN"); // to evaluate expressions written in Reverse Polish Notation since the parsing is easier to implement starting off
    kwds.push("dumpstack"); // dumps stack into a variable and resets stack
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
    bin_num_ops.insert("%".to_string(), |x, y| x % y);
    let bin_num_ops = bin_num_ops;
    let elapsed = now.elapsed();
    println!("\n===== Binary Numerical Operators: =====\n\n{:?}\n", bin_num_ops.keys().cloned().collect::<Vec<String>>());
    println!("Elapsed: {:.2?}\n", elapsed);

    // define language stack numerical operators
    let now = Instant::now();
    let mut stac_num_ops: HashMap<String, fn(&Vec<f32>)->f32> = HashMap::new();
    stac_num_ops.insert("$".to_string(), |stac| sum(stac));
    stac_num_ops.insert("£".to_string(), |stac| prod(stac));
    let stac_num_ops = stac_num_ops;
    let elapsed = now.elapsed();
    println!("\n===== Stack Numerical Operators: =====\n\n{:?}\n", stac_num_ops.keys().cloned().collect::<Vec<String>>());
    println!("Elapsed: {:.2?}\n", elapsed);

    let ops = "+*-/%$£";

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

    // execution takes place from here on
    println!("\n===== SCRIPT EXECUTION: =====\n\n");
    let mut init_placeholder: Vec<&str>;

    for ln in content.iter(){
        println!("Line : {:?}\n", ln);
        if ln.contains(" : num = ") {
            init_placeholder = ln.split(" : num = ").collect::<Vec<&str>>();
            num_table.insert(init_placeholder[0], init_placeholder[1].to_string().parse::<f32>().unwrap());
            println!("Numtable : {:?}\n", num_table);
        } else if ln.contains("RPN : ") {
            let expr = ln.strip_prefix("RPN : ").unwrap();
            println!("{:?}\n", expr);
            let var_name = expr.split(" = ").collect::<Vec<&str>>()[1];
            let expr = expr.split(" = ").collect::<Vec<&str>>()[0];
            println!("parse : {:?}\n{:?}\n", var_name, expr);
            let placeholder: f32 = eval_rpn(&(expr.to_string()), &num_table, &bin_num_ops, &stac_num_ops, &ops);
            num_table.insert(var_name, placeholder);
            println!("Numtable : {:?}\n", num_table);
        }else {

        }

    }


    println!("\n===== SCRIPT EXECUTION COMPLETE =====\n\n");

}
