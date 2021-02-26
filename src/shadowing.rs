fn main() {
    let x: i32 = 4;
 
    {
        let x: i32 = 5;
        println!("Aquí x es {}",x)
    }
    println!("X es {}",x)
    //Shadowing
 }
 