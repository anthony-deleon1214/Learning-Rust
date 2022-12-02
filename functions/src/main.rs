fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    // Functions implicitly return the value of the last expression
    // An expression does not end in a semicolon, so no semicolon is included here
    // This function returns 5 as a 32-bit signed integer
    5
}