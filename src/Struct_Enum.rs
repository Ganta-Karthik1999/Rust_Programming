enum milkshakes{
    
    chocolate,
    venala,
    strawberry,

}

struct drink{

    milk:milkshakes,
    fuild:f64,
}



fn display(temp: drink){
    match temp.milk{

        milkshakes::chocolate=>println!("Chocolate"),
        milkshakes::venala=>println!("venala"),
        milkshakes::strawberry=>println!("strawberry"),
    }

    println!("{:?}",temp.fuild);

}


fn main() { 

// Here we have a enum function that is called in a Structure so in order to use it 
//We need to fisrt define the stuctructure

let chocolate=drink{
    milk:milkshakes::chocolate,
    fuild:10.0,
};

display(chocolate);

let venala=drink{

    milk:milkshakes::venala,
    fuild:20.0,
    
    };

    display(venala);

let strawberry = drink{

    milk:milkshakes::strawberry,
    fuild:30.0,

};
display(strawberry);

}
