## Common Programming Concepts

### Variables and Mutability

Variables in rust are immutable by default. To declare a variable as mutable, the use of `mut` keyword is required.

```rust
fn main() {
    let x = 5;      // immutable variable
    let mut y = 6;  // mutable variable
}
```

Variables can be shadowed by the latest declaration of the variable with the same name.

```rust
fn main() {
    let x = 5;
    let x = 6;
}
```

Constants are containers that bounds to a value upon initialization and cannot be changed, not even with the magical `mut` keyword. Constants can be declared in both global and local scope of the program.

```rust
const PI: f64 = 3.1415926535;

fn main() {
    const E: f64 = 2.71818281828;
}
```

### Data Types

Rust is a statically typed language. The compiler needs to know all the types of the variables at compile time. This can be done by inferring or explicitly provide a type to the variable.

The example below shows the compilation error when Rust can't infer the types based on the input provided.

```rust
fn main() {
    // compilation error: E0282
    let tryInferMe = "66".parse().expect("Not a correct number.");
    
    // compiled successfully
    let tryInferMe: u32 = "66".parse().expect("Not a correct number");
}
```

#### Scalar Types

A _scalar type_ represents a single value. There are four scalar types in Rust, primarily **integers**, **floating points**, **booleans** and **characters**.

##### Integer Types

Integer is a number without the fractional part. Below shows a tabulated representation of the possible integer types in Rust.

| Length | Signed | Unsigned |
| --- | --- | --- |
| 8-bit | `i8` | `u8` |
| 16-bit | `i16` | `u16` |
| 32-bit | `i32` | `u32` |
| 64-bit | `i64` | `u64` |
| 128-bit | `i128` | `u128` |
| arch | `isize` | `usize` |

Signed integers are stored using two's complement. Each signed numbers can store numbers starting from $-2^{n-1}$ to $2^{n-1}-1$. For instance, `i8` can store numbers from $-2^7$ to $-2^7-1$ (-128 to 127) whereas `u8` can store numbers from 0 to $2^8-1$ (255).

`isize` and `usize` are types that depends on the architecture of the computer that **the program is running on**.

Integer Literals can be represented in many forms listed down in the table below.

| Number literals | Example |
| --- | --- |
| Decimal | `98_222` |
| Hex | `0xff` |
| Octal | `0o77` |
| Binary | `0b1111_0000` |
| Byte (`u8` only) | `b'A'` |

> If an attempt to hold values outside the range of the type, _integer overflow_ will occur.

##### Floating Points

```rust
fn main() {
    let x = 2.0;        // f64
    let y: f32 = 2.0;   // f32
}
```

##### Boolean

```rust
fn main() {
    let x = true;
    let y: bool = false;
}
```

##### Characters

Characters are enclosed by single quotes as opposed to double quotes used by string literals. Characters can represent more than just ASCII. Accented letters, Chinese, Japanese and Korean characters and emoji are all supported with the character type in Rust.

```rust
fn main() {
    let z = 'z';
    let z: char = 'ℤ';
    let z = '😊';
}
```

#### Compound Types

_Compound types_ are composed by multiple scalar types. Rust has **tuples** and **arrays** as compound types.

##### Tuple

Tuple is used for grouping multiple values into one. It has a fixed length and the size cannot be changed once initialized.

```rust
fn main() {
    // declaring tuple
    let tuple: (i32, f64, u8) = (100, 3.14, 1);
    
    // destructuring
    let (num1, num2, num3) = tuple;
    
    // access elements via period `.` and index of the element
    let first = tuple.0;
    let third = tuple.2;
}
```

The tuple without any elements `()` are called **unit**. Expressions return unit implicitly to signify that no values has been returned.

##### Array

Rust's array have a fixed length. Arrays are useful when you know the numbers of elements that won't be changed.

```rust
fn main() {
    // inferring types
    let a = [1, 2, 3, 4, 5];
    
    // explicit types
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // declare array of same elements
    let a = [3; 5]; // [3, 3, 3, 3, 3]
}
```

Elements within an array can be referred the same way as other programming language via indexing.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let first = a[0];
    let second = a[1];
}
```

Accessing invalid elements in an array will result in runtime error.

### Functions

Functions are just a chunk of code that can be reused again and again. It starts with a `fn` keyword followed by the name of the function and the arguments for the function.

```rust
fn hello(name: &str) {
    println!("Hello {}", name);
}
```

#### Statements and Expressions

Functions are made up of lines of statements and optionally end with an expression. Statement doesn't return values.

#### Returning Values

Values can be returned in a function by putting them in the last line of the block and **don't** end with the semicolon `;`. Optionally specifies the return type of the function explicitly by chaining `->` after the function parentheses.

```rust
fn calculate(x: i32, y: i32) -> i32 {
    x + y
}
```

### Control Flows

#### If Expressions

If statement allows the code the branch of depending on the condition specified. Contrary to other popular programming languages, the expression in the if statement does not need to be encapsulated by parentheses. Also worth noting is the block that associated with an if condition is sometimes referred to as _arms_.

Of course, the `else` clause of an if statement is optional, we can omit them if they are not applicable.

```rust
fn main() {
    let x = 5;
    if x < 6 {
        println!("x is lesser than 6!");
    } else if x < 9 {
        println!("x is lesser than 9!");
    } else {
        println!("x is greater than 9!");
    }
}
```

##### Using if in a let Statement

Recall that blocks in Rust (encapsulated by curly braces) can be used to return values by ending the block with some expression without a semicolon. The same applies to an if else statement and below illustrates an example.

```rust
fn main() {
    let x = 5;
    
    let y = if x > 6 { true } else { false };
}
```

Using such assignment will need to ensure that both (or more) of the arm returns the same data type or the compilation will fail.

#### Repetition with Loops

Rust has three kinds of loops, namely `loop`, `while` and `for`.

##### Using loop

The `loop` keyword repeats the code defined within its block indefinitely. The only way to break out of the loop is by specifying an exit condition with an if statement.

```rust
fn main() {
    // Code are for illustration purposes only
    loop {
        println!("Looping forever");
    }
    
    let mut x = 0;
    
    loop {
        println!("Loop until condition fulfilled");
        
        if x == 6 {
            break;
        }
    }
}
```

The loop can be broken out along with values too by providing the value after the `break` keyword as such.

```rust
fn main() {
    let mut count = 0;
    let b = loop {
        count += 1;
        
        if count > 6 {
            break count * 2;
        }
    };
    
    println!("The result is {b}");
}
```

##### Loop Labels to Disambiguate Between Multiple Loops

Sometimes it can be hard to distinguish between loops when there are a few of them nested within each other. Fortunately, Rust provides labels that allow us to specifically break out the loop we intended to.

```rust
fn main() {
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```

##### While loop

While loop basically resembles its counterparts in other programming languages.

```rust
fn main() {
    let mut counter = 0;
    
    while counter < 5 {
        println!("counter: {counter}");
        counter += 1;
    }
}
```

##### For loop

The for loop in Rust is analogous to the Foreach loop in JavaScript and C#. It basically allows us to loop through each of the individual element within a collection.

```rust
fn main() {
    let a = [10, 20, 30, 40, 60];
    for element in a {
        println!("Element is {a}!");
    }
}
```

The range operator can be used together with for loop too.

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
```

### Reference and Appendix

- [Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)

