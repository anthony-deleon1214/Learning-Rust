fn main() {
    let x = 5; // Defining x with main scope
    // Repeating the let keyword allows shadowing (A new variable x is created for this scope)
    let x = x + 1; // Shadowing x in main scope (x now equals 6)

    {
        let x = x * 2; // Shadowing x in an internal scope (x equals 12)
        println!("The value of x in the inner scope is: {x}");
    }

    // Since internal scope has been exited, x returns to its value before entering internal scope (x equals 6)
    println!("The value of x is: {x}");
}
