fn coordinate()->(i32,i32){

    (1,7)

}
fn main() {
// Assigning  an tuple with a value
    let first =(2,3);
    println!("{:?}, {:?}", first.0,first.1);
    println!("================================================");
    let (x,y)=first;
    println!("{:?}, {:?}",x,y);
    // Now we use Struct and tuple together  to optimize the code 
println!("================================================");
    let (x,y)=coordinate();
    println!("{:?}, {:?}",x,y);

    if y>5{
        println!(">5");
    }
    else{
        println!("<5");
    } 

    // In Tuple we can combine different types of data
    println!("================================");
    let (name,age)=("Karthik",23);
    println!("{:?}",name);

}


