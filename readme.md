# arya.

[crates.io](https://crates.io/crates/arya) · [docs.rs](https://docs.rs/arya/0.0.2/arya/) · `arya = "0.0.2"`

simple json validation. simple json repair. lightning fast.

## example: json validation + repair.

```rust
let mut builder = JsonBuilder::new();

builder.update(r#"{
    "name": "annie",
    "age": 14,
    "parents": {
        "mother": null,
        "broken
"#);

builder.update("value");

builder.completed_string()

// => Ok({
// =>     "name": "annie",
// =>     "age": 14,
// =>     "nested": {
// =>         "mother": null
// =>     }
// => })
```

# example: json validation

```rust
let mut json = JsonVerifier::new();

for character in r#"{ "name": "annie", "value": 1 }"#.bytes() {
    println!(
        "{} - {:?} - {:?}",
        character as char,
        json.update(character),
        json.status());
}

//     { - Ok(()) - Continue
//       - Ok(()) - Continue
//     " - Ok(()) - Continue
//     n - Ok(()) - Continue
//     a - Ok(()) - Continue
//     m - Ok(()) - Continue
//     e - Ok(()) - Continue
//     " - Ok(()) - Continue
//     : - Ok(()) - Continue
//       - Ok(()) - Continue
//     " - Ok(()) - Continue
//     a - Ok(()) - Continue
//     n - Ok(()) - Continue
//     n - Ok(()) - Continue
//     i - Ok(()) - Continue
//     e - Ok(()) - Continue
//     " - Ok(()) - Continue
//     , - Ok(()) - Continue
//       - Ok(()) - Continue
//     " - Ok(()) - Continue
//     v - Ok(()) - Continue
//     a - Ok(()) - Continue
//     l - Ok(()) - Continue
//     u - Ok(()) - Continue
//     e - Ok(()) - Continue
//     " - Ok(()) - Continue
//     : - Ok(()) - Continue
//       - Ok(()) - Continue
//     1 - Ok(()) - Continue
//       - Ok(()) - Continue
//     } - Ok(()) - Valid
```
