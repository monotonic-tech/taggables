//! Unit tests for [taggables].
#![cfg(test)]

use std::fmt::Display;
use std::marker::PhantomData;

use taggables::*;

taggables! {
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct TaggedString(String);

    #[derive(Debug, Clone)]
    pub(crate) struct ComplexType<T>(u64, String, T);

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct WithPublicField(pub u64);

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct EmptyParens();

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct NoParens;
}

semantic_tags! {
    FirstNameTag("FirstName"),
    SurnameTag("Surname"),
    NameOverride("NameOverrideWorks"),
    NoOverride
}

type FirstName = TaggedString<FirstNameTag>;
type Surname = TaggedString<SurnameTag>;

impl<S: SemanticTag> Display for TaggedString<S> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple(S::name()).field(&self.0).finish()
    }
}

impl<S: SemanticTag> TaggedString<S> {
    pub fn new(inner: impl Into<String>) -> Self {
        Self(inner.into(), PhantomData)
    }
}

mod retag;
mod semantic_tags_macro;
mod taggables_macro;
mod wrapper;

#[test]
fn tag_name_reports_the_tag_semantic() {
    assert_eq!(FirstName::tag_name(), "FirstName");
    assert_eq!(<Tagged<String, SurnameTag>>::tag_name(), "Surname");
}

#[test]
fn unit_and_infallible_are_semantic_tags() {
    assert_eq!(<()>::name(), "()");
    assert_eq!(<core::convert::Infallible>::name(), "!");
}
