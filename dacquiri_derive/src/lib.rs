//! An authorization framework with compile-time enforcement.
//!
//! `Dacquiri-derive` makes using `Dacquiri` ergonomic.
//!
//! For more information on `Dacquiri`, check out its crate documentation!

use proc_macro::TokenStream;

mod subject;
mod attribute;
mod entitlement;

/**
Marks a struct as being a `Subject`.

`Subject` structs can acquire `Attributes` resulting in them gaining entitlements.

To use this derive, simply annotate a struct with the macro.

```rust
use dacquiri::prelude::Subject;

#[derive(Subject)]
struct User {
    username: String
}
```
*/
#[proc_macro_derive(Subject)]
pub fn subject_macro(input: TokenStream) -> TokenStream {
    subject::handle_subject(input)
}

/**
Marks a trait as being an _entitlement_.

_Entitlements_ are implemented on any _subject_ that has acquired all of the required _attributes_. Attributes
can be specified with the `entitlement` macro like so.

```rust
#[entitlement(AttributeOne, AttributeTwo)]
pub trait MyEntitlement {
    fn do_thing(&self) {
        println!("This subject has AttributeOne and AttributeTwo!");
    }
}
```
*/
#[proc_macro_attribute]
pub fn entitlement(args: TokenStream, input: TokenStream) -> TokenStream {
    entitlement::handle_entitlement(args, input)
}

/**
Marks a function as an _attribute_.

_Attributes_ define properties that we want to test on _subjects_.

For a _subject_ to hold an _attribute_ it must pass the condition defined in the function. See the
crate level documentation for more information on how to use `#[attribute]`
*/
#[proc_macro_attribute]
pub fn attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::handle_attribute(args, input)
}
