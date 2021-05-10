# Data types in Rust
Rust is a statically typed language. It must know the types of all variables at compile time. The compiler can usually infer the type based on use and context, so it assign a default type if we dont. 



## 1. Scalar

Scalar types represents a single value. Rust has 4 scalar types:ç

* Integers
* Floating-point
* Booleans
* Characters



### 1.1 Scalar - Integer

There are 2 basic types: Signed and unsigned. Signed means that it has a sign (+ or -) so the value can be positive or negative. While unsigned means that its only positive and does not allow negative values.

Because signed values must allow negative values, it allows the middle of range positive numbers. For example:

If we assign 8 bits with `i8` or `u8` 
i8: -128 to 127
u8: 0 to 255

The signed will cover the half of positive numbers because it need to allow save the negatives in the same space of memory.


|Length	|Signed	|Unsigned
--- 	| --- 	| ---
|8 bit	|i8		|u8
|16 bit	|i16	|u16
|32 bit	|i32	|u32
|64 bit	|i64	|u64
|128 bit|i128	|u128
|arch	|isize	|usize

`isize` and `usize` are special types that take the architecture of the system (32 bit or 64 bit).

So `isize` will be `i32` in a 32bit system but `i64` in a 64bit system.

The default type is `i32` because it stores 64 bits (good for 64bit based systems) and it allows negative numbers too.



### 1.2 Scalar - Floating point

Rust has 2 types for floating point numbers: `f32` and `f64`.

The default is `f64`, again because it's as fast as `f32` on 64bit based systems, the most common nowadays, but giving the double of space allowing more precision.

```rust
fn main() {
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32
}
```



### MATHS inside Rust - Basic operations

Rust supports the basic operations: addition, substraction, multiplication, division and reminder.

```rust
fn main() {
    
    let sum = 5 + 10; // addition

    let difference = 95.5 - 4.3; // subtraction

    let product = 4 * 30; // multiplication
    
    let quotient = 56.7 / 32.2; // division

    let remainder = 43 % 5; // remainder
}
```



### 1.3 Scalar - Boolean

As in most other programming languages. A boolean that can store `true` or `false` value.

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```



### 1.4 Scalar - Character

Rust supports single letters too. The `char` type represents a Unicode Scalar Value. 

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let emoji = '😊';
}
```



---



## 2. Compound

Compound types can group multiple values into one type. Rust has 2 primitive compound types: Tuples and Arrays.



### 2.1 Compound - Tuple

A tuple is a general way of grouping a number of values. Tuples have a fixed length: once declared, they cannot grow or shrink in size. The type of each value doesn't need to be the same.

```rust
fn main() {
    let tup: (i32, f64, u8) = (50, 6.4, 1);
}
```

 The variable `tup` binds to the entire tuple, being considered a single compound element. We can destructure the tuple to get the individual values:

```rust
fn main() {
    let tup = (50, 6.4, 1);
    let (x, y, z) = tup;
}
```

Also, we can access a single element like:

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let six_point_four = x.1;
}
```



### 2.2 Compound - Array

Another way of grouping data. Unlike a tuple, every element of the array must have the same type. And important: **Arrays in Rust are different from arrays in some other languages because arrays in Rust have fixed length, like tuples**.

```rust
let a = [1, 2, 3, 4, 5];
```

There is another collection type: Vectors. They are more flexible than Arrays (they can grow and shrink in size). If you are not sure when to use an Array or a Vector, then use a Vector.

An example of when to use Arrays (fixed in size) could be if we are storing months in our program for example. It's very unlikely thay such program will need to add or remove months, so you can use an array.

```rust
let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
```

To declare the type of the array, use square brackets. Declare the data type, and after a semicolon the number of elements.

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

That syntax looks similar to the syntax for initialize an array with default values, first declaring the initialValue for all the values of the array, and then declaring the length.

```rust
let a: [3; 5];
// This equals to:
// let a = [3, 3, 3, 3, 3];
```

To access an array element, use indexing:

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let first = a[0]
let second = a[1]
```









