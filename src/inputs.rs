use std::io;

fn main(){
    let mut input = String::new();

    println!("Hey, di algo: ");

    match io::stdin().read_line(&mut input){
        Ok(_) => {println!("Perfecto, digiste {}", input)},
        Err(e) => println!("Ocurrio un error... \n {}",e)
    }
}