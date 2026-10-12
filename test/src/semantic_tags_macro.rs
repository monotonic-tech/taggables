use taggables::SemanticTag as _;
use taggables::Taggable as _;
use taggables::Tagged;

taggables::taggables! {
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct TaggedString(String);
}

taggables::semantic_tags! {
    FirstNameTag("FirstName"),
    SurnameTag("Surname"),
}

taggables::semantic_tags! {
    NameOverride("NameOverrideWorks"),
    NoOverride
}

/// The test is in this compiling without error
pub mod generated_tags_do_not_raise_missing_docs_warning {
    #![deny(missing_docs)]
    taggables::semantic_tags! {
        pub TagWithNoDoc,
    }
}

#[test]
fn no_name_override_defaults_to_type_name() {
    assert_eq!(NoOverride::name(), "NoOverride");
}

#[test]
fn name_override_works() {
    assert_eq!(NameOverride::name(), "NameOverrideWorks");
}

type FirstName = TaggedString<FirstNameTag>;

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
