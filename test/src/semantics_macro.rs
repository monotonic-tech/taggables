extern crate alloc;

use alloc::string::String;
use core::marker::PhantomData;

use taggables::{Taggable, Tagged};

struct Digest<const L: usize, S = ()>([u8; L], PhantomData<fn() -> S>);

impl<const L: usize, S> Taggable for Digest<L, S> {
    type Tag = S;
    type This<NewTag> = Digest<L, NewTag>;

    fn retag<NewTag>(self) -> Self::This<NewTag> {
        Digest(self.0, PhantomData)
    }
}

taggables::semantics! {
    text_semantics for Tagged {
        pub Email<String>,
        pub Username<String>,
        pub(crate) Internal<String>,
        Private<String>,
        #[cfg(any())]
        Disabled<MissingType>,
    }
}

pub mod nested {
    use taggables::Tagged;

    taggables::semantics! {
        local for Tagged {
            pub(crate) CrateOnly<u32>,
            pub(super) ParentOnly<u32>,
            pub(in crate) AncestorOnly<u32>,
        }
    }
}

// test is this compiling
const _: fn() = || {
    let _ = Internal::new(String::new());
    let _ = Private::new(String::new());
    let _ = nested::ParentOnly::new(1);
    let _ = nested::CrateOnly::new(1);
    let _ = nested::AncestorOnly::new(1);
};

#[test]
fn crate_visibility_allows_access_from_a_sibling_module() {
    assert_eq!(nested::CrateOnly::new(7).into_inner(), 7);
}

#[test]
fn private_alias_is_usable_in_its_defining_scope() {
    assert_eq!(Private::new("private".into()).into_inner(), "private");
}

#[test]
fn parent_visibility_is_relative_to_the_invocation() {
    assert_eq!(nested::ParentOnly::new(7).into_inner(), 7);
}

#[test]
fn ancestor_visibility_is_preserved() {
    assert_eq!(nested::AncestorOnly::new(7).into_inner(), 7);
}

#[test]
fn nested_type_argument_is_preserved() {
    taggables::semantics! {
        tags for Tagged { Names<Vec<String>> }
    }

    assert_eq!(
        Names::new(vec![String::from("Alice")]).into_inner(),
        ["Alice"]
    );
}

#[test]
fn reference_type_argument_is_preserved() {
    taggables::semantics! {
        tags for Tagged { Name<&'static str> }
    }

    assert_eq!(Name::new("Alice").into_inner(), "Alice");
}

#[test]
fn qualified_type_argument_is_preserved() {
    taggables::semantics! {
        tags for Tagged { Name<alloc::string::String> }
    }

    assert_eq!(Name::new(String::from("Alice")).into_inner(), "Alice");
}

#[test]
fn const_argument_is_preserved() {
    taggables::semantics! {
        tags for Digest { Hash<28> }
    }

    let hash: Hash = Digest([7; 28], PhantomData);
    assert_eq!(hash.0.len(), 28);
}
