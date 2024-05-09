enum Direction{
    Left,
    Right,
    Up,

} 

enum Color{
    Red,
    Green,
    Blue,
}


fn print_color(color: Color) {
    match color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
fn main() {
    
    let go =Direction :: Up;
    match go {
        Direction::Left => println!("left"),
        Direction::Right => println!("right"),
        Direction::Up => println!("up"),
    }
    print_color(Color::Blue);
}
