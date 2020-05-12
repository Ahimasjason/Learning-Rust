extern crate greprs;
use std::io::prelude::*;
use std::env::args;
use std::process;
use greprs::Config;
use greprs::FileReader;
use std::fs;


fn write_error(err_str :String){
    let mut stderr = std::io::stderr();
    writeln!(
        &mut stderr,
        "{}",
        err_str
    ).expect("Could not write to stderr");
}
fn main(){
    // Accepting args from command line
    // let args: Vec<String> = args().collect();
    // let mut stderr = std::io::stderr();
    // /home/simsol/Downloads/crack_code_inter_test.pdf
    // String::from("/home/simsol/Downloads/crack_code_inter_test.pdf"),String::from("find")
    
    
    let con = Config::new(args()).unwrap_or_else( |err| 
        {
            write_error(format!("Unable to create config {}",err));
            process::exit(1);
    });
   
    if let false = greprs::check_file_name(&con.filename){
        
        write_error(format!("Not a valid file format expected (txt,pdf), pass only pdf or text file"));
        process::exit(1);
    }
    //  Check file exists 
    if let false = greprs::file_exists(&con.filename){
        
        write_error(format!("Given file not found on the path"));
        process::exit(1);
    };

    println!("{:?}",con);
    // println!("{:?}",args);
    let mut fs = match FileReader::new(&con.filename){
            Ok(file) => file,
            Err(_) => {
                write_error(format!("Given file not found on the path"));
                process::exit(1);
            }
    };

    // let fs = FileReader::new(&con.filename).expect();
    fs.search_str(&con.query_string);
    println!("{:?}",fs);
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