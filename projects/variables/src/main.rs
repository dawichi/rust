fn main() {
    // const names are written with uppercase and _ as convention
    // numers can be written with _ to improve readability
    const NUMER_TO_PRINT: u32 = 10_000;
    println!("The value of x is: {}", NUMER_TO_PRINT);

    // variables are inmutable by default
    let var_inmutable = 5;
    println!("The value of x is: {}", var_inmutable);

    // to allow mutability, just set 'mut' keyword before the var name
    let mut var_mutable = 10;
    println!("The value of x is: {}", var_mutable);
    var_mutable = 15; 
    println!("The value of x is: {}", var_mutable);

    // also, you can "shadow" a inmutable variable
    let var_inmutable = "hello";
    // by shadowing the previous value, it also allows to modify the data type
    // when with a mutable variable it would throw an error
}

