// struct GroceryItems{
//     Stock:i32,
//     Price:f64,


// }

// fn main(){

//     let Items=GroceryItems{
//         Stock:12,
//         Price:3.2,

//     };
//     println!("================================================================");
//     println!("The stock Present are==>{:?} and the Price of the each Item is ===>{:?} ",Items.Stock,Items.Price);




// }

enum Flavour{
    chocolate,
    Red_velvet,


}

struct Cake{
    flavor :Flavour,
    quantity: f64,
}

fn print_flavor(a:Cake){

    match a.flavor{
        Flavour::Red_velvet=>println!("Red velvet cake"),
        Flavour::chocolate=>println!("Chocolate cake"),

    }
    println!("The Quantity that we are using is {:?}",a.quantity);
}


fn main(){

let sweet=Cake{

    flavor: Flavour::chocolate,
    quantity:1.0,
};
println!("============================================================");
print_flavor(sweet);

let sweet=Cake{

    flavor :Flavour::Red_velvet,
    quantity:2.0,
};
println!("============================================================");
print_flavor(sweet);
}












































