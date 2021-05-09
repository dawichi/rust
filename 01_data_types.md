# Data types in Rust
Rust is a statically typed language. It must know the types of all variables at compile time. The compiler can usually infer the type based on use and context, so it assign a default type if we dont. 


## 1. Scalar

Scalar types represents a single value. Rust has 4 scalar types:ç

* Integers
* Floating-point
* Booleans
* Characters

<br>

---

<br>
  
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

<br>

---

<br>

### 1.2 Scalar - Floating point

Rust has 2 types for floating point numbers: `f32` and `f64`.

The default is `f64`, again because it's as fast as `f32` on 64bit based systems, the most common nowadays, but giving the double of space allowing more precision.

```rust
fn main() {
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32
}
```

<br>

---

<br>

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

<br>

---

<br>

### 1.3 Scalar - Boolean

As in most other programming languages. A boolean that can store `true` or `false` value.

```rust
fn main() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}
```


