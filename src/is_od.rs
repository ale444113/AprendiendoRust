fn main() {
    let number_to_check: i32  = 4;

    if is_od(number_to_check){
        println!("{} is odd",number_to_check);
    }else{
        println!("{} isn't odd",number_to_check)
    }
}

fn is_od(n: i32) -> bool{
    if n % 2 == 0{return true;}
    return false;
}
