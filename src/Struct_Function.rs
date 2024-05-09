struct GroceryItem{
    stock: i32,
    price: f64,
}


fn main() {
    let c= GroceryItem{
        stock: 10,
        price:2.99,


    };
println!("{:?}", c.stock);
println!("{:?}", c.price);

}
