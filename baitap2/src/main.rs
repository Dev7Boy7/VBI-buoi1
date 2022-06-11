use std::io::stdin;
use std::fs;

fn main() {
    // Input String outside 
    let contents = fs::read_to_string("baitap2.txt").expect("Something went wrong");

    // Input your key WORD 
    println!("Please input your WORD :");

    let mut input_str = String::new();
            stdin().read_line(&mut input_str).unwrap();

    println!("Your searching Key-Word : {}", &input_str);

    //Return the number of repeat

    let abc = contents.matches(&input_str.trim()).count();

    println!("The number of repeat : {} ", abc);

}

