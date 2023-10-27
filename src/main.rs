fn main() {
    // Shadowing
    println!("\n--- Start Shadowing ---");
    shadowing();
    println!("--- End Shadowing ---\n");

    // Ownership
    println!("\n--- Start Ownership ---");
    let mainOwnedString: String = String::from("Hello Owner");
    println!("{}", mainOwnedString); 
    ownership(mainOwnedString.clone()); // use String.clone() to create a new instance of the string
    println!("{}", mainOwnedString);
    println!("--- End Ownership ---\n");
}


fn shadowing() {
    let var1 = 2;
    println!("before assignment var1 = {}", var1);

    let var1 = 3;
    println!("after shadowing var1 = {}", var1);
}


fn ownership(unowned: String) {
    println!("{}", unowned);
}
