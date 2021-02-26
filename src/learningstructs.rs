struct Person{
    name: String,
    years: i32,
    phone: String
} 
trait IsSmall{
    fn is_baby(&self) -> bool;
}
impl ToString for Person{
    fn to_string(&self) -> String{
        return format!("Here is your info \n Name: {} \n You are {} old \n Phone number: {} \n", self.name, self.years, self.phone);
    }
}
impl IsSmall for Person{
    fn is_baby(&self) -> bool{
        if self.years <= 4{
            return true;
        }return false;
    }
}
fn main() {
    let mut ale = Person { name: "Ale".to_string(), years: 14, phone: "+34 000 000 000".to_string()};
    println!("{}",ale.to_string());
    println!("One year later... \n");
    ale.years += 1;
    println!("{}",ale.to_string());
    println!("{} is baby? -> {}",ale.name, ale.is_baby())
}