struct Triangle{
    width: i32,
    height: i32
} 
impl Triangle{
    fn print_description(&self){
        println!("Here is the info of this triangle... \n Width: {} \n Height: {}", self.width, self.height);
    }
    fn is_rectangle(&self) -> bool{
        self.height == self.width
    }
}
fn main() {
    let my_triangle = Triangle { width: 14, height: 15};
    my_triangle.print_description();
    println!("This triangle is rectangle: {}", my_triangle.is_rectangle());    
}
