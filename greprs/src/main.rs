extern crate greprs;
use std::io::prelude::*;
use std::env::args;
use std::io::prelude::*;
use std::process;
use std::env;
use greprs::Config;
use std::fs;




fn main(){
    // Accepting args from command line
    let args: Vec<String> = args().collect();
    let mut stderr = std::io::stderr();
    // /home/simsol/Downloads/crack_code_inter_test.pdf
    let con = Config::new(String::from("/home/simsol/Downloads/crack_code_inter_test.pdf"),String::from("find")).unwrap_or_else( |err| 
        {
            writeln!(
                &mut stderr,
                "problem parsing args : {} ",
                err
            ).expect("Could not write to stderr");
            process::exit(1);
    });
   
    if let false = greprs::check_file_name(&con.filename){
        writeln!(
            &mut stderr,
            "Not a valid file format expected (txt,pdf), pass only pdf or text file"
        ).expect("Could not write to stderr");
        process::exit(1);
    }
    //  Check file exists 
    if let false = greprs::file_exists(&con.filename){
        writeln!(
            &mut stderr,
            " Given file not found on the path "
        ).expect("Could not write to stderr");
        process::exit(1);
    };
    println!("{:?}",con);
    println!("{:?}",args);
}



// fn main(){
//     let number = Some(5);
//     let mut sec : Option<i32> = None;
//     if let Some(i) = number{
//         println!("Number is not None and number is {}", i);
//     }
//     sec = Some(7);
//     if let None = sec {
//         println!(" sec is None so raise error");
//     }
// }