enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("It's High"),
        false => println!("It's Low"),
    }
}

fn main() {
    // Secret File: Admin only here we used the Enum and expression together
    let access_level = Access::Admin;
    let can_access_file = match access_level {
        Access::Admin => true,
        Access::Manager => true, // Using Manager variant
        Access::User => true,    // Using User variant
        Access::Guest => false,  // Using Guest variant
    };
    println!("===================================");
    println!("Can Access {:?}", can_access_file);

    // Now we will use If else in short hand
    let value = 100;
    let is_gt_100 = value > 100;

    print_message(is_gt_100);
}
