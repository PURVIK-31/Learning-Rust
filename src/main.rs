struct Rectangle{
    width : u32,
    height:u32,
}
impl Rectangle{
    fn getarea(&self)->u32{
        return self.width * self.height;
    }
    fn getperimeter(&self)->u32{
        return 2*(self.width + self.height);
    }
    fn display(&self){
        println!("Area of rectangle is : {}",self.getarea());
        println!("Perimeter of rectangle is : {}",self.getperimeter());
    }
    fn displayr(){
        println!("Thanks for code execution!");
    }
}
fn main(){
let rect1 = Rectangle{
    width:30,
    height:50,
};
rect1.display();
Rectangle::displayr();
}