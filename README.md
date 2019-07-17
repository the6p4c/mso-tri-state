# `mso-tri-state`: fearless booleans

Gone are the days where a simple true/false boolean variable suffices. Modern software requires
modern solutions: `MsoTriState`.

Trusted by Microsoft.

## Old, slow, ancient, unsafe code
```rust
let foo = true;
if foo {
    println!("Hello, world!");
}

// Hard to read, intent unclear
let bar = 1 == 2;
match bar {
    false => println!("One does not equal two"),
    true => println!("One equals two"),
    // Restrictive, not web-scale
}
```

## New, fast, web-scale, safe code
```rust
extern crate mso_tri_state;
use mso_tri_state::MsoTriState;

// Clean and easy to read
let foo = MsoTriState::msoTrue;
if foo.into() {
    println!("Hello, world!");
}

// Simple, effortless conversion
let bar: MsoTriState = (1 == 2).into();
match bar {
    MsoTriState::msoFalse => println!("One does not equal two"),
    MsoTriState::msoTrue => println!("One equals two"),
    // Highly future-proof and scalable
    _ => panic!(),
}

// Compatible with all major brands
let has_a_3 = MsoTriState::from(vec![1, 2, 4, 5].contains(&3));
println!("Has a 3: {}", has_a_3); // prints "Has a 3: msoFalse"
```
