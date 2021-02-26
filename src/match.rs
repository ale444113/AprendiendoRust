fn main(){
    let number = 3;

    match number{
        1 => println!("The number is 1"),
        2 => println!("The number is 2"),
        3...10 => println!("The number is between 3 an 10"),
        11 | 12 => println!("The number is 11 or 12"),
        _ => println!("It doesn't match")
    }
}