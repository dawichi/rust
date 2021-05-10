# Functions in Rust

Functions are pervasive in Rust code. The `main` function is the entry point of many programs, declared with `fn` keyword. Rust uses *snake case* as convenctional style for function and variable names. 

Functions usually have *parameters* or *arguments*.

```rust
fn main() {
    add_function(5, 6);
}

fn add_function(x: i32, y: i32) {
    let result: i32 = x + y;
    println!("The result is: {}", result);
} 
```



## Function bodies: Statements and Expressions

Function bodies are made up of a series of statements, optionally ending in an expression.

Main function with a let expression
```rust
fn main() {
    let y = 6;
}
```

Statements do not return values. So you can't assign a `let` statement to another variable. This will get an error:

```rust
fn main() {
    let x = (let y = 6);
}
```

This is because `let y = 6;` does not return any value, so `x` can't bind anything. In other languages such as C or Ruby, the assignment returns the value of the assignment, so you can write `x = y = 6`. But Rust is different.

Expressions evaluate to something. A simple math operation like `5 + 6` is an expression that evaluates to the value `11`. Expressions can be part of statements like with `let x = 5 + 6;`. Calling a function is an expression. Calling a macro is an expression. The block to create new scopes `{}` is an expression.

For example:

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        // x+1 is an expression
        x + 1
    }

    println!("The value is: {}", x)
}
```



## Functions with return values

They are declared with `->`. All the functions return the value of the final expression in the block of the body function. You can return an early expression with the `return` keyword.

```rust
fn main() {
    let x = plus_one(5);
    println!("The value is: {}", x);
}

fn plus_one(x: i32) -> {
    x + 1
}
```

Realize that `x + 1` inside `plus_one` does not have a `;`. This is because it's a expression, not a statement. If we put the semicolon in the end of the line, the compiler will interpret it as a statement so we'll get an error:

```
implicitly returns `()` as its body has no tail or `return` expression
    x + 1;
- help: consider removing this semicolon
```


The error message reveals the issue: the definition of the function says that it will return an `i32`, but the statements don't evaluate to a value.