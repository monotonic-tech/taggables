use taggables::{SemanticTag, Taggable};
use taggables_tests::public_fixtures::Email;

#[test]
fn public_alias_is_usable_from_another_crate() {
    let email = Email::new("alice@example.com".into());
    assert_eq!(email.into_inner(), "alice@example.com");
}

#[test]
fn public_alias_preserves_its_semantic_name() {
    assert_eq!(Email::tag_name(), "Email");
    assert_eq!(<Email as Taggable>::Tag::name(), "Email");
}
