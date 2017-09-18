# Program notes

## Documentation

The first thing I notice is that, even though you have gone to the trouble to add comments for your program, `cargo doc` does not produce any documentation for your crate. This is because you've used /* comments */ instead of Rust's doc comments.

Doc comments in Rust come in two forms.

/// Documentation for a struct or method.

//! Documentation for a crate.

The first option will appear in the documentation for just that struct or function, while the second appears as basically the intro section when you view a crate's documentation. An example of rustdoc in action is doc.rust-lang.org/std/ (if I remember correctly) or everything at docs.rs.

## Organization

Naturally, I would probably break this into several smaller functions. I usually just don't like reading long ones, because there's often a lot of complexity in a longer function definition that I would rather not think about while trying to understand the function as a whole.

## Error codes

This is nitpicky, but my preference would be that you name your error codes. I'm not saying I do that, but as a guy reading your program, that would be helpful. :) An alternative would be to return a strongly typed error object of some kind from your archive function and then produce an error code from that in main.

By "named" error codes, I mostly just mean using a constant, like `const HARD_DRIVE_EXPLODED: u32 = 1;` or something like that. I don't remember if error codes are u32; I'm just scribbling this off the top of my head.

If you like the idea of a strongly typed error, my preferred pattern is to use something like the following:

```rust
use std::error;

struct Error {
    kind: ErrorKind,
    cause: Option<Box<error::Error>>,
}

#[derive(Copy, Clone)]
enum ErrorKind {
    HardDriveExploded,
    FileContainsStateSecrets,
    FileNotFound,
}
```

Then you can match on that kind enum later on to derive your error code:

```rust
use std::process;

match error.kind {
    ErrorKind::FileContainsStateSecrets => process::exit(1), // The police are coming,
    ErrorKind::HardDriveExploded => process::exit(2), // No police for this one
    ErrorKind::FileNotFound => process::exit(3), // I think this police are coming for this one, too.
}
```

As a final note on error codes, you might have found out something different in testing, but I *believe* that a Rust program will naturally emit a code of zero on termination unless you tell it to do something else.

## ToString

For converting a string slice or a static string into an owned string, my preference as a reader is the form `String::from("hello")`, because that tips off the reader at the very start of the line as to what's going on.

## Paths

It's kind of intimidating, when you first get introduced to rust, to learn that strings and paths are not the same thing anymore. However, the difference between a path and a string applies only one way. All strings can be valid paths; it's just that not all paths can be valid strings, since strings are also required to be UTF-8. As a result, strings and string slices have been given a neat little AsRef implementation that allows them to be referenced as paths. Meaning that you can skip all that making-paths-out-of-strings nonsense.

## Command line arguments

Personally, I would just go ahead and collect them into a vector rather than iterating them multiple times. I have no reason for saying this; it's not like I've done performance testing to see which is better. An alternative would be to iterate just the one time and keep a reference to the iterator throughout so that you can use bits as you need them.

For anything not a toy, I usually use `clap` for reading command line arguments. It isn't necessarily *harder* to use than just doing it yourself--if you're going to do any kind of error handling, even in a toy, it may well be easier to use `clap` instead.
