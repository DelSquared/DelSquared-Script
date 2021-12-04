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

    // define table for numerical variables
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

    // define table for boolean variables
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
    println!("\n===== Binary Numerical Operators: =====\n\n{:?}\n", bin_num_ops.keys().cloned().collect::<Vec<String>>());

    // define language logical operators
    let mut bin_log_ops: HashMap<String, fn(bool,bool)->bool> = HashMap::new();
    bin_log_ops.insert("&".to_string(), |x,  y| x & y);
    bin_log_ops.insert("|".to_string(), |x,  y| x | y);
    bin_log_ops.insert("^".to_string(), |x,  y| x ^ y);
    bin_log_ops.insert("!".to_string(), |x, _y| !x );
    let bin_log_ops = bin_log_ops;
    println!("\n===== Logical Operators: =====\n\n{:?}\n", bin_log_ops.keys().cloned().collect::<Vec<String>>());

    // define language comparative operators
    let mut bin_comp_ops: HashMap<String, fn(f32,f32)->bool> = HashMap::new();
    bin_comp_ops.insert("<=".to_string(), |x, y| x <= y);
    bin_comp_ops.insert(">=".to_string(), |x, y| x >= y);
    bin_comp_ops.insert("==".to_string(), |x, y| x == y);
    bin_comp_ops.insert("!=".to_string(), |x, y| x != y);
    bin_comp_ops.insert("<".to_string(),  |x, y| x <  y);
    bin_comp_ops.insert(">".to_string(),  |x, y| x >  y);
    let bin_comp_ops = bin_comp_ops;
    println!("\n===== Binary Comparative Operators: =====\n\n{:?}\n", bin_comp_ops.keys().cloned().collect::<Vec<String>>());

    // define language control operators
    let mut bin_ctrl_ops: HashMap<String, i32> = HashMap::new();
    bin_ctrl_ops.insert("=".to_string(), 0); // initialise
    bin_ctrl_ops.insert("~".to_string(), 1); // destroy
    let bin_ctrl_ops = bin_ctrl_ops;
    println!("\n===== Control Operators: =====\n\n{:?}\n", bin_ctrl_ops.keys().cloned().collect::<Vec<String>>());



}
