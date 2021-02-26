use std::fs::File;
use std::io::prelude::*;

fn main(){
    //let mut file = File::open("something.txt").expect("Couldn't open the file");
    let mut file = File::create("something.txt").expect("Couldn't create the file");

    file.write_all("Hello world").except("Could not write the file");

    let mut content = String::new();
    file.read_to_string(&mut content).expect("Couldn't read the content of the file");
    
    println!("{}",content);
}