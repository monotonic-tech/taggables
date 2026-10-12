# Taggables

![Crates.io Version](https://img.shields.io/crates/v/taggables)
![docs.rs](https://img.shields.io/docsrs/taggables)
[![CI](https://github.com/monotonic-tech/taggables/actions/workflows/ci.yml/badge.svg)](https://github.com/monotonic-tech/taggables/actions/workflows/ci.yml)

Tagged types à rebours for Rust.

<img src="https://raw.githubusercontent.com/monotonic-tech/taggables/main/taggables.png"
     alt="A corny looking 'Taggables' logo in a similar style to 'Lunchables', I think, idk, I eat normal food."/>

## Who is tagged type?

Tagged types are types that carry an additional, usually phantom,
type parameter ("semantic tag"). Two instantiations of the same base
type with different semantic tags are considered distinct by the
compiler. This prevents values with different meaning but otherwise
compatible underlying data types from being mixed up.

For example, this is perfectly valid Rust code:
```rust
fn mount(rider: &str, ridee: &str) { /* 🏇💨 */ }

// oh no!
mount("horse", "jockey");
```

In contrast, this is not:
```rust ignore
use taggables::{Tagged, WrapWithTag};

taggables::semantic_tags! { Rider, Ridee }

fn mount(rider: Tagged<&str, Rider>, ridee: Tagged<&str, Ridee>) { /* 🏇💨 */ }

// ❌ error[E0308]: expected `Tagged<&str, Rider>`, found `Tagged<&str, Ridee>`
mount(
    "horse".wrap_with_tag::<Ridee>(),
    "jockey".wrap_with_tag::<Rider>(),
);
```

## The Switcheroo

The way tagged types are usually implemented is by introducing a wrapper
type that takes both the tag and the type it is applied to:

```rust
pub struct Tagged<Inner, Tag>(Inner, std::marker::PhantomData<Tag>);

pub enum NameTag {}
type Name = Tagged<String, NameTag>;
```

This is the path taken by the [tagged-types][1] and [rust-tagged][2]
crates, and it has the important advantage of being applicable to external
types such as the `String` in the example above. However, this comes at
the price of wrapping/unwrapping and occasional run-ins with the compiler
due to the wrapper not being a perfect substitute for the inner type.

Taggables tries to avoid these issues completely for owned types, by making
the base type itself carry the semantic tag:
``` rust
pub struct TaggableString<Tag>(String, std::marker::PhantomData<fn(Tag) -> Tag>);

pub enum NameTag {}
type Name = TaggableString<NameTag>;
```
Okay, this example might not look that impressive, as we still end up with
just a `String` wrapper, but just think about any type that is primitive
but polysemic in *your* domain: a `ByteString32([u8; 32])` can be a `PublicKey`
or a `PrivateKey`. It would be a shame if someone swapped them, wouldn't it?

## The API

A taggable base type carries an additional phantom parameter and implements
[Taggable], semantic tags implement [SemanticTag]:

``` rust
use std::marker::PhantomData;

pub struct TaggableString<Tag>(String, PhantomData<fn(Tag) -> Tag>);

impl<T> taggables::Taggable for TaggableString<T> {
  type Tag = T;
  type This<S> = TaggableString<S>;

  fn retag<S>(self) -> Self::This<S> {
      TaggableString(self.0, PhantomData)
  }
}

pub enum EmailTag {}

impl taggables::SemanticTag for EmailTag {
  fn name() -> &'static str {
    "Email"
  }
}

pub type Email = TaggableString<EmailTag>;
```

That's it, there's no magic here. Okay, there's also [taggables] and [semantic_tags]
macros to cut out some of this boilerplate:

```rust
taggables::taggables! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct TaggableString(String);
}

taggables::semantic_tags! {
    pub EmailTag("Email"),
}

pub type Email = TaggableString<EmailTag>;
```

And [semantics] to declare tags and aliases together:

```rust
taggables::taggables! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct TaggableString(String);
}

taggables::semantics! {
    text_semantics for TaggableString {
        pub Email<String>,
        pub Username<String>,
    }
}
```

## Revenge of the Wrapper

Alas, you cannot just stick an additional type param onto a foreign type and call it
yours, which means the wrapper approach is here to stay, thanks to the same orphan rule
that we all know and love.

Luckily, the traditional wrapper shape fits perfectly into the taggables API:

``` rust
use std::marker::PhantomData;

pub struct Tagged<Inner, Tag>(Inner, PhantomData<fn(Tag) -> Tag>);

impl<T, S> taggables::Taggable for Tagged<T, S> {
    type Tag = S;

    type This<S2> = Tagged<T, S2>;

    fn retag<S2>(self) -> Self::This<S2> {
        Tagged::<T, S2>(self.0, PhantomData)
    }
}
```

And so, taggables which was supposed to destroy wrapping, joins it instead with its own
[`Tagged`] wrapper:

``` rust
use taggables::{ WrapWithTag, Tagged };

taggables::semantic_tags! {
    pub Message("Message"),
}

let important_message_to_luke: Tagged<String, Message> =
    "I am your father".to_string().wrap_with_tag::<Message>();

assert_eq!(important_message_to_luke.into_inner(), "I am your father");
```

Nooooooooooo.

## License

[Apache License, Version 2.0][LICENSE-APACHE] or [MIT license][LICENSE-MIT].

Any contribution submitted to this crate by you shall be dual licensed as above,
without any additional terms or conditions. You've been warned.

---

[1]: https://github.com/poroh/tagged-types
[2]: https://github.com/codefonsi/rust-tagged

[LICENSE-APACHE]: https://github.com/monotonic-tech/taggables/blob/main/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/monotonic-tech/taggables/blob/main/LICENSE-MIT

[Taggable]: https://docs.rs/taggables/latest/taggables/trait.Taggable.html
[SemanticTag]: https://docs.rs/taggables/latest/taggables/trait.SemanticTag.html
[taggables]: https://docs.rs/taggables/latest/taggables/macro.taggables.html
[semantic_tags]: https://docs.rs/taggables/latest/taggables/macro.semantic_tags.html
[`Tagged`]: https://docs.rs/taggables/latest/taggables/struct.Tagged.html
