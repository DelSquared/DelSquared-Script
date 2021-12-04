use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    // commandline args
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("\n===== In file: =====\n\n{}\n", filename);

    // read file specified by commandline args
    let content = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("\n===== With text: =====\n\n{}\n", content);

    // parse lines and strip empty (i.e. useless) lines
    let mut content: Vec<&str> = content.split("\n").collect();
    while let Some(ln) = content.iter().position(|x| *x == "\r") {
        content.remove(ln);
    }
    while let Some(ln) = content.iter().position(|x| *x == "") {
        content.remove(ln);
    }
    println!("\n===== Split text: =====\n\n{:?}\n", content);

    // define table for variables
    let mut var_table: HashMap<&str, f32> = HashMap::new();

    // example variable initialisation (remove later)
    println!("\n===== Var table tests: =====\n\n");
    println!("{:?}\n", var_table);
    var_table.insert("test_var", 100.0);
    println!("{:?}\n", var_table);
    var_table.insert("test_var2", 10.1);
    println!("{:?}\n", var_table);
    var_table.remove("test_var");
    println!("{:?}\n", var_table);
    var_table.remove("test_var2");
    println!("{:?}\n", var_table);

    // define regex patterns for parsing... Might not keep this idk
    let mut parse_patterns: Vec<&str> = Vec::new();
    parse_patterns.push(r"(\w*)\((.*)\)"); //function
    let parse_patterns: Vec<&str> = parse_patterns;
    println!("\n===== Keywords: =====\n\n{:?}\n", parse_patterns);

    // define language keywords
    let mut kwds: Vec<&str> = Vec::new();
    kwds.push("if");
    kwds.push("else");
    kwds.push("end");
    kwds.push("label");
    kwds.push("goto");
    let kwds: Vec<&str> = kwds;
    println!("\n===== Keywords: =====\n\n{:?}\n", kwds);

    // define language binary numerical operators
    let mut bin_num_ops: HashMap<String, fn(f32,f32)->f32> = HashMap::new();
    bin_num_ops.insert("+".to_string(), |x, y| x + y);
    bin_num_ops.insert("-".to_string(), |x, y| x - y);
    bin_num_ops.insert("*".to_string(), |x, y| x * y);
    bin_num_ops.insert("/".to_string(), |x, y| x / y);
    let bin_num_ops = bin_num_ops;
    println!("\n===== Numerical Operators: =====\n\n{:?}\n", bin_num_ops.keys().cloned().collect::<Vec<String>>());

    // define language binary logical operators
    let mut bin_log_ops: HashMap<String, fn(bool,bool)->bool> = HashMap::new();
    bin_log_ops.insert("&".to_string(), |x, y| x & y);
    bin_log_ops.insert("|".to_string(), |x, y| x | y);
    bin_log_ops.insert("^".to_string(), |x, y| x ^ y);
    let bin_log_ops = bin_log_ops;
    println!("\n===== Logical Operators: =====\n\n{:?}\n", bin_log_ops.keys().cloned().collect::<Vec<String>>());

    // define language comparative operators
    let mut bin_log_ops: HashMap<String, fn(f32,f32)->bool> = HashMap::new();
    bin_log_ops.insert("<=".to_string(), |x, y| x <= y);
    bin_log_ops.insert(">=".to_string(), |x, y| x >= y);
    bin_log_ops.insert("==".to_string(), |x, y| x == y);
    bin_log_ops.insert("!=".to_string(), |x, y| x != y);
    bin_log_ops.insert("<".to_string(),  |x, y| x <  y);
    bin_log_ops.insert(">".to_string(),  |x, y| x >  y);
    let bin_log_ops = bin_log_ops;
    println!("\n===== Logical Operators: =====\n\n{:?}\n", bin_log_ops.keys().cloned().collect::<Vec<String>>());

    // define language control operators
    // let mut bin_log_ops: HashMap<String, fn(f32,f32)->bool> = HashMap::new();
    // bin_log_ops.insert("=".to_string(), |x, y| x <= y);
    // bin_log_ops.insert("~".to_string(), |x, y| x >= y);
    // let bin_log_ops = bin_log_ops;
    // println!("\n===== Logical Operators: =====\n\n{:?}\n", bin_log_ops.keys().cloned().collect::<Vec<String>>());


}
