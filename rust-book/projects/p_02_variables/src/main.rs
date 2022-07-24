fn main() {
	// ------------ CONSTS ------------
    // const names are written with uppercase and _ as convention
    // numers can be written with _ to improve readability
    const NUMER_TO_PRINT: u32 = 10_000;
    println!("The value of x is: {}", NUMER_TO_PRINT);

	// ------------ VARIABLES ------------
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

	// Rust recommends use shadowing instead of mutable variables because that
	// provides the erros if we try to modify the variable without shadowing by error.
	// And another important difference, is tha with shadowing we are creating a new var
	// so we are able to modify the data type, while with mutable we are not
	let x = 5;
	let x = x + 1;
	let x = x * 2;
	// If we try:    x = x + 1
	// we'd get a compile-time error because x is not mutable

	// another useful example, if we want allow the user to insert a number of spaces
	// with shadowing we can reuse the variable name. First is String type and we transform it into a integer
	let spaces = "    "
	let spaces = spaces.len()

	// while with mutability we would need to do something like
	let mut spaces = "    "
	let spaces_count = spaces.len()

}

