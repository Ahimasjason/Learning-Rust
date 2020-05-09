extern crate greprs;

use greprs::Config;

fn main(){
    println!("Working");
    let con = Config{filename:String::from("File name"), query_string : String::from("find")};
    println!("{:?}",con)
}