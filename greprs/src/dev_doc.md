# Greprs Implentation In Rust

---

## TO DO":"

---
- [x] Create the struct that store following
  - [x] File name
  - [x] Query String
- [x] provide checks to check file exists or not 
  - [x] Done.  
- [x] Accept arguments from command line
  - [x] Provide validation for file Accept only (pdf,txt file)
- [x] Read the file 
- [x]  provide search method on file reader method that accept seach string as query string
- [x]  Create the new txt file that containing query string line
- [x]  save the file in that same directory with nre name 

Target:-
===
>   Upload the file from the web(rocket) and   ask what kind of query string user wants to retrive  
> from that file and finally give that query sting avalilable lines as file.  

---

## Syntax
  * if let statement : 
    * > ** if let Err(e) = res{  
           println!("Application Error {} " ,err) }