enum Shape {
    Circle(u32),
    Square(u32, u32),
    Rectangle(u32, u32),
}

fn main() {
    let shape1 = Shape::Circle(2); // Circle with radius 2
    let shape2 = Shape::Square(2, 2); // Square with sides 2
    let shape3 = Shape::Rectangle(2, 2); // Rectangle with width 2 and height 2

    match shape1 {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Square(width, height) => println!("Square with width {} and height {}", width, height),
        Shape::Rectangle(width, height) => println!("Rectangle with width {} and height {}", width, height),
    }
    let areaof: u32 = area(shape1);
    println!("Area of shape1 is {}", areaof);
    let areaof: u32 = area(shape2);
    println!("Area of shape2 is {}", areaof);       
    let areaof: u32 = area(shape3);
    println!("Area of shape3 is {}", areaof);
}

fn area(s:Shape)->u32{
    match s{
        Shape::Circle(r)=>3*r*r,
        Shape::Square(w,h)=>w*h,
        Shape::Rectangle(w,h)=>w*h,
    }
}