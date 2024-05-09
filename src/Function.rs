fn first_name(){
    println!("Karthik");
}

fn last_name(){
    println!("Ganta");
}

fn sub(a:i32,b:i32){
    println!("================");
    println!("{:?}",a-b);

}


fn main() {
 first_name();
 last_name();

 let sum=2+2;
 let diff=5-3;
 let devide=25/5;
 let mul=5*5;
 println!("============================");
 println!("{:?} {:?} {:?} {:?}",sum,diff,devide,mul);

 sub(8,4);

let even_number = 6%3;
let  odd_number= 6%4;
println!("============================");
println!("{:?}, {:?}",even_number,odd_number);
}
