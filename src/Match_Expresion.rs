fn main() {
    let some_bool=true;
    match some_bool{

        true=>println!("its true!!!!"),
        false=>println!("It's not true"),
    }
//  Or the other way we can do it like

let sample=3;
match sample{

    1=>println!("it's 1"),
    2=>println!("it's 2"),
    3=>println!("it's 3"),
    _=>println!("it's something else "),
}

// The other way is 

let my_name="karthik";

match my_name{
    "karthik"=>println!("That is my name"),
    "Bob"=>println!("This is my not my name"),
    "hari"=>println!("This is also not my name"),
    _=>println!("Nothing"),
}

}
