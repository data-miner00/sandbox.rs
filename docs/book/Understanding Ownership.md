## Understanding Ownership

Ownership is a unique feature and concept in Rust that ensures memory safety without the use of garbage collector.

### What is Ownership?

Ownership is a set of rules imposed to manage memory without depending on garbage collector in Rust. 

We say that the object being created by the context that it is in, has the ownership of that particular object. The ownership of an object is delegated to the context that it is being used, and no longer valid on the context that it was initially created. That way, we can make sure that there are no more than one reference to the same object at the same time and this drastically reduces the bugs that might arise due to multiple referencing.

#### Ownership Rules

- Each value has an owner
- There can only be one owner at a time
- When owner gets out of scope, the value will be dropped.

#### Variable Scope

```rust
{                      // s is not valid here, it’s not yet declared
    let s = "hello";   // s is valid from this point forward

    // do stuff with s
}   // this scope is now over, and s is no longer valid
```

The variable is valid when it comes into scope, but is invalidated when the code goes out of scope. To keep the variable alive, we can consider returning the variable with its ownership to the calling context and assign a variable there.

```rust
let a = {
    let s = "hello";
    s
}
```

The variable `a` will get the full ownership of the inner variable `s` with its value `"hello"` preserved intact.

#### The String Type

String type is a good example for illustrating ownership as it is a more complex data structure that cannot be trivially copied like those of primitives. The value of the string data type are stored in heaps and the actual size of it might not be known at compile time, for example when taking inputs from user.

#### Memory and Allocation

String literals are fast and efficient because it is hardcoded and the size is determined upon compile time. However, since `String` type will change and mutate along the lifetime of the program, a definite storage of fixed size can be less useful. 

The `drop` function is automatically called against any of the variables by Rust at the end of scope. This can be referred to _Resource Acquisition Is Initialization (RAII)_ in C++.

#### Move

Moving is also a part of ownership mechanism where reassigning a reference type actually transfer the ownership of the object, rendering void for the previous container that holds the object.

```rust
fn main() {
    // primitive
    let x = 5;
    let y = x;
    // x = 5, y = 5
    
    // reference types
    let w = String::from("hello");
    let v = w;
    // w -> invalid, v = String::from("hello")
}
```

Another reason for the ownership transfer is to avoid the _double free error_ where when the code goes out of scope, both `v` and `w` will `drop()` the same object, which is illegal.

> Note: Rust will never automatically deep copy our data because it might be expensive to pull out.

#### Clone

Creating a deep copy of the heap data is possible with `.clone()` method.

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    // both s1, s2 valid with same value
}
```

#### Stack-Only Data: Copy

Stack-only data (a.k.a primitives) will always copy because it is cheap to do so. Rust also disallows to implement the `Copy` traits for types that has implemented the `Drop` traits.

An explicit list of types that implement `Copy`:

- All integer types
- Boolean types
- Floating Points
- Character Type
- Tuples that only contain types that implements the `Copy` traits

#### Ownerships and Functions

Functions take ownerships of the reference type that is being passed in by default.

```rust
fn main() {
    let x = String::from("hello");
    
    random_function(x); // takes ownership of x
    
    // x no longer valid here
}
```

#### Return Values and Scope

Similarly, function can return an ownership too.

```rust
fn main() {
    let x = random_function(); // function that takes 0 args and return a string
}
```

### References and Borrowing

Fortunately, it is possible to pass in an object type without transferring the ownership. We can achieve this via reference.

```rust
fn main() {
    let s1 = String::from("hello");
    
    takes_reference(&s1); // ownership never transferred to the function
    
    // s1 is still valid here
}
```

The parameters in the function definition of `take_reference` will of course needs to be declared as taking in reference instead. Reference can be declared by prefixing an ampersand `&` in front of the type.

```rust
fn take_references(str: &String) {
    // do smth with str
}
```

#### Mutable References

The references we've discussed so far is read-only. We cannot modify the content within the context where the type is being referenced. However, we can use mutable reference `&mut` to achieve this.

```rust
fn main() {
    let mut s1 = String::from("hello");
    
    do_thing_without_take_ownership(&mut s1);
}

fn do_thing_without_take_ownership(str: &mut String) {
    str.push_str(", world");
}
```

There are one restriction when it comes to using mutable references and immutable references. There can be multiple immutable reference at a single scope but there can always be one and only one mutable reference at any scope. Furthermore, immutable reference and mutable reference cannot be present in a single scope.

```rust
fn valid() {
    let a = String::from("hello");
    
    let x = &a;
    let y = &a;
}

fn invalid_one() {
    let mut a = String::from("hello");
    
    let x = &mut a;
    let y = &mut a; // invalid, cannot borrow mutable more than once
}

fn invalid_two() {
    let mut a = String::from("hello");

    let x = &a;
    let y = &a;
    let z = &mut a; // invalid, 
}
```

However, if the immutable references is intercepted by function calls that transfer the ownerships, immutable reference can be used within the same scope.

```rust
fn main() {
    let mut s = String::from("hello");
    
    let a = &s;
    let b = &s;
    
    println!("{} and {}", a, b); // ownership transferred
    
    let c = &mut s; // no problem
}
```

Rust imposes this restriction to prevent race condition at compile time.

#### Dangling References

Programming languages that make extensive use of points are bound to create a _dangling pointer_, which is a prone to erroneous behaviour down the line. Dangling pointer can be best described as a pointer that references a location in memory they might belong to others (such as previously owned but given to someone else later). The operation is still valid as the pointer to that particular location has not been reset or disconnected. Below is an epitome of how dangling pointer would happen in the context of Rust.

```rust
fn main() {
    let void_reference = dangle();
}

fn void_reference() {
    let s = String::from("hello");
    
    &s
}
```

Essentially, the `void_reference` is returning the reference or a pointer in this context to the caller. However, the variable `s` will be dropped once the function goes out of scope, hence the returned reference to the content of that position in memory is deallocated.

### The Slice Type

Slices allow us to reference a contiguous sequence of element in a collection rather than the whole item. As slice is a reference type, ownership does not apply and can be used carefree.

String is a collection of bytes. The function to extract the first word from a sentence would be as follows.

```rust
fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    
    str.len()
}
```

The `iter()` creates an iterator over the array of bytes and `enumerate()` wraps the element into a tuple with index.

In essence, the above function will create a variable to hold the returned value, which indicates the index of the first blank space. With that, we have a generated value which is tightly coupled to the string, where the returned index doesn't make sense on its own without working with the original string that generated it.

Managing these values are error-prone and tedious to say the least as managing thousands of these in a massive project is unsustainable.

#### String Slices

Here comes String Slices. It has a very intuitive syntax that look as such.

```rust
let s = String::from("hello wolrd");

let hello = &s[0..5];
let world = &s[6..11];
```

The `..` range syntax allows easy manipulation of indexing a string into string slices. 

| Word | Syntax | Result |
| --- | --- | --- |
| mountain | `&s[0..5]` | mount |
| mountain | `&s[..5]` | mount |
| mountain | `&s[2..]` | untain |
| mountain | `&s[..]` | mountain |
| mountain | `&s[0..s.len]` | mountain |

This technique can be used to improve the function earlier.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}
```

#### String Literals Are Slices

As string literals are stored in binary, they are referred as slices.

```rust
let s = "hello, world";
```

The `s` are of type `&str`, a slice that points to the specific point in binary, hence string slices are immutable.

#### String Slices As Parameters

String and string slices can be passed into the parameter of type `&str`.

```rust
fn first_word(word: &String) -> &str {}

fn first_word(word: &str) -> &str {}
```

The above behaviour uses _deref coercion_ for the added flexibility.

Below showcase the coercion in action.

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

### Other Slices

A more general slices are also available for other data types. The operation to refer to the slice of value is the same.

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```



