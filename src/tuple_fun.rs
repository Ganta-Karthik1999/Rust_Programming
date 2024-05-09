enum Acess{

    full,
}

fn one_two_three()->(i32, i32,i32){
    (1,2,3)

}


fn main() {
    let number=one_two_three();
    let (x,y,z)=one_two_three();
    println!("{:?} {:?} ",x,number.0);

 let (employ,access)=("Karthik",Acess::full);

}
 