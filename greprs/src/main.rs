extern crate greprs;
use std::env; //bringing the enviroment
use std::process;

use greprs::Config;

// #[derive(Debug)]
// struct Config {
//     query_str: String,
//     filename :String,
// }
// impl Config {
//     // add code here
//     // new will return Result enum 
//         // 
//     fn new(args:&[String])->Result<Config, &'static str>{
//         if args.len()< 3{
//             return Err("Not enough arguments");
//         }
//         let query = args[1].clone();
//         let f_name = args[2].clone();
//         Ok(Config{query_str:query, filename:f_name})
//     }

//     fn cng_config(&mut self){
//         self.query_str = String::from("Different");
//     }
// }
fn main() {
    let args: Vec<String> = env::args().collect();
    // ["target/debug/greprs", "search_string", "file_name.txt"] 
    // println!("{:?}", args);
    // let search_string = &args[1];
    // let file_name1 = &args[2];
    // parse_config(&args);
    // let mut config :Config = Config::new(&args)?; // this does not work in main
    let mut config :Config = Config::new(&args).unwrap_or_else(|err| {
        println!("We gotta problem {}",err);
        process::exit(1); // sys.exit()
    });
    // config.cng_config();
    println!("{:?}",config);
    if let Err(err) = greprs::run(config){
        println!("Application Error {}",err);
        process::exit(1);
    };
    // println!("Search Sting is {}",search_string);
    // println!("File name is {} ", file_name);
    // let content = fs::read_to_string(config.filename) // this will always read root level file.
    //     .expect("Something went wrong");
    // println!("{:?}",content);
}

// fn parse_config(args : &[String]) -> Config {
//     let query_str = args[1].clone();
//     let filename = args[2].clone();
//     // println!("{} {}",query_str,filename);
//     Config{query_str,filename }
// }


// fn run(config:Config) -> Result<(),Box<dyn  Error>>{
//     /// dyn --> Dynamic
//     /// Box<dyn Error> means the function will return a type that implements the Error trait, 
//     /// but we don’t have to specify what particular type the return value will be. 
//     /// This gives us flexibility to return error values that may be of different types in different 
//     /// error cases. The dyn keyword is short for “dynamic.”
//     let content = fs::read_to_string(config.filename)?;
//     println!("\n{}",content);
//     Ok(())
// }