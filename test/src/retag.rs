use std::marker::PhantomData;

use taggables::SemanticTag;
use taggables::Taggable as _;

taggables::semantic_tags! {
    FirstNameTag("FirstName"),
    SurnameTag("Surname"),
}

taggables::taggables! {
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct TaggedString(String);

    #[derive(Debug, Clone)]
    pub(crate) struct ComplexType<T>(u64, String, T);

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct EmptyParens();

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct NoParens;
}

impl<S: SemanticTag> TaggedString<S> {
    pub fn new(inner: impl Into<String>) -> Self {
        Self(inner.into(), PhantomData)
    }
}

type FirstName = TaggedString<FirstNameTag>;
type Surname = TaggedString<SurnameTag>;

#[test]
fn retag_works() {
    let retagged_john = FirstName::new("John").retag::<SurnameTag>();
    let surname_john = Surname::new("John");

    assert_eq!(retagged_john, surname_john);
}

#[test]
fn retag_into_works() {
    let retagged_john = FirstName::new("John").retag_into::<Surname>();
    let surname_john = Surname::new("John");

    assert_eq!(retagged_john, surname_john);
}

#[test]
fn retagging_works_with_multiple_fields() {
    let value: ComplexType<bool> = ComplexType(7, "data".into(), true, PhantomData);
    let retagged = value.retag::<FirstNameTag>();

    assert_eq!(retagged.0, 7);
    assert_eq!(retagged.1, "data");
    assert!(retagged.2);
}

#[test]
fn untag_replaces_the_tag_with_unit() {
    let untagged = FirstName::new("John").untag();

    assert_eq!(untagged, TaggedString::<()>("John".into(), PhantomData));
}

#[test]
fn fieldless_types_retag() {
    assert_eq!(
        EmptyParens::<FirstNameTag>(PhantomData).retag::<SurnameTag>(),
        EmptyParens(PhantomData)
    );
    assert_eq!(
        NoParens::<FirstNameTag>(PhantomData).retag::<SurnameTag>(),
        NoParens(PhantomData)
    );
}
