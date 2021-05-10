## If

NOTE: Using too many `else if` expressions can clutter your code. It's better to use another branching construct: `match`.

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    }
    else if number % 3 == 0 {
        println!("number is divisible by 3");
    }
    else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    else {
        println!("number is not divisible by 4, 3 or 2");
    }
}
```





## If inside let

Because `if` is a expression, we can use it inside a `let` statement.


```rust
fn main() {
    let condition = true
    let number = if condition { 5 } else { 6 };
    println!("The number values: {}", number); // 5
}
```

Note that both `if` and `else` arms must have same data types. This will get an error:

```rust
fn main() {
    let condition = true
    let number = if condition { 5 } else { "six" }; // ERROR!
    println!("The number values: {}", number); 
}
```





## Repetition

Rust has three kind of looping:

* `loop`
* `while`
* `for`



### Loop

Repeats a block again and again until you explicitly tell it to stop.

````rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
````



### While

For conditional loops. When the condition ceases to be true, the program calls `break`, stoping the loop. This kind of loop type could be implemented using a combination of `loop`, `if`, `else`, and `break`. However, this pattern is so common that Rust has a built-in language construct for it called `while`.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```



### For

Looping through a collection with for. 

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```





#### Extra: countdown with 'for' and 'rev()'

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```







