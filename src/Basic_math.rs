//Basic Arthematic
// Sum 
fn sum(a: i32,b: i32)-> i32{

    a+b
}

 fn display_result(result:i32){

    println!("{:?}",result);
}

fn main() {
    let r=sum(2,3);
    display_result(r);
}
