mod maps;

fn main() {
    // Shadowing
    println!("\n--- Start Shadowing ---");
    shadowing();
    println!("--- End Shadowing ---\n");

    // Ownership
    // Doesn't apply to 'copy' type data types such as numbers and booleans

    println!("\n--- Start Ownership ---");
    let main_owned_string: String = String::from("Hello Owner");
    println!("{}", main_owned_string); 
    ownership(main_owned_string.clone()); // use String.clone() to create a new instance of the string
    println!("{}", main_owned_string);
    println!("--- End Ownership ---\n");

    // Borrowing
    println!("\n--- Start Borrowing ---");
    let main_owned_string: String = String::from("Hello Borrower");
    println!("{}", main_owned_string); 
    borrowing(&main_owned_string); // the String cannot be mutated when used like this
    println!("{}", main_owned_string); 

    let mut main_owned_string = String::from("Hello Borrower");
    println!("{}", main_owned_string);
    mut_borrowing(&mut main_owned_string); // the String CAN be mutated when used like this
    println!("{}", main_owned_string); 
    println!("--- End Borrowing ---\n");

    // Modules
    println!("\n--- Start Modules ---");
    let new_string: String = maps::get_something_crazy();
    println!("Printing {}", new_string);
    println!("--- End Modules ---\n");
}


fn shadowing() {
    let var1 = 2;
    println!("before assignment var1 = {}", var1);

    let var1 = 3;
    println!("after shadowing var1 = {}", var1);
}


fn ownership(string: String) {
    println!("{}", string);
}

fn borrowing(string: &String) {
    println!("{}", string);
}

fn mut_borrowing(string: &mut String) {
    string.push_str(" ~~~ I have been mutated ~~~");
    println!("{}", string);
}
