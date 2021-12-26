use std::collections::HashMap;

pub fn sum(stack: &Vec<f32>) -> f32{
  stack.iter().sum()
}

pub fn prod(stack: &Vec<f32>) -> f32{
  stack.iter().product()
}

pub fn eval_rpn(ln: &str, numtable: &HashMap<&str, f32>, bin_num_ops: &HashMap<String, fn(f32,f32)->f32>, stac_num_ops: &HashMap<String, fn(&Vec<f32>)->f32>, ops: &str) -> f32 {
    let mut stack: Vec<f32> = Vec::new();
    let line_parse = ln.split(' ').collect::<Vec<&str>>();
    let mut place_holder: f32;

    for t in line_parse.iter(){

      println!("tkn = {:?}",t);
      if ops.contains(t){
        if bin_num_ops.contains_key(&t.to_string()){
            place_holder = bin_num_ops[&t.to_string()](stack[0], stack[1]);
        } else {
            place_holder = stac_num_ops[&t.to_string()](&stack);
        }
        stack = Vec::new();
        stack.push(place_holder);
        println!("{:?}",stack);

    } else if t.chars().all(|x| x.is_numeric()){
        stack.push(t.to_string().parse::<f32>().unwrap());
        println!("{:?}",stack);
    } else {
        let varname : String = t.to_string().to_owned();
        stack.push(numtable[&varname[..]]);
        println!("{:?}",stack);

    }

    }
    println!("result = {:?} (quick maths)",stack);

    stack[0]
}
