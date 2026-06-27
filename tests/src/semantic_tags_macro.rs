use super::*;

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
