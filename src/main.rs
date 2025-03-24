#[derive(Debug)]
#[allow(dead_code)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Center,
}

fn main(){
let player_direction = Direction::Right;
match player_direction {
    Direction::Up => println!("Moving up!"),
    Direction::Down => println!("Moving down!"),
    Direction::Left => println!("Moving left!"),
    Direction::Right => println!("Moving right!"),
    _ => println!("Standing still!"),
}
}