enum Color{
    Red,
    Yellow,
    Blue,

}

fn print_color(my_color:Color){

    match my_color{
       Color::Red=> println!("Red"),
       Color::Yellow=> println!("Yellow"),
       Color::Blue=> println!("Blue"),

    }
}

fn main() {

    print_color(Color::Blue);
    print_color(Color::Red);
    print_color(Color::Yellow);
    
    
    
}
