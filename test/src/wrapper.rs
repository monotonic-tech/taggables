use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use taggables::Tagged;
use taggables::WrapWithTag as _;

taggables::semantic_tags! {
    FirstNameTag("FirstName"),
    SurnameTag("Surname"),
}

#[test]
fn wrap_with_tag_round_trips_with_into_inner() {
    let wrapped = "John".to_string().wrap_with_tag::<FirstNameTag>();

    assert_eq!(wrapped.as_ref(), "John");
    assert_eq!(wrapped.into_inner(), "John".to_string());
}

#[test]
fn new_is_equivalent_to_wrap_with_tag() {
    let wrapped = "John".to_string().wrap_with_tag::<FirstNameTag>();
    let constructed = Tagged::<String, FirstNameTag>::new("John".to_string());

    assert_eq!(wrapped, constructed);
}

#[test]
fn default_delegates_to_the_inner_default() {
    let defaulted: Tagged<u8, FirstNameTag> = Tagged::default();

    assert_eq!(defaulted, Tagged::<u8, FirstNameTag>::new(0));
}

fn hash(value: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

#[test]
fn hashing_delegates_to_the_inner_value() {
    let wrapped = "John".to_string().wrap_with_tag::<FirstNameTag>();

    assert_eq!(hash(wrapped), hash("John".to_string()));
}

#[test]
fn the_tag_does_not_change_the_hash() {
    let first_name = "John".to_string().wrap_with_tag::<FirstNameTag>();
    let surname = "John".to_string().wrap_with_tag::<SurnameTag>();

    assert_eq!(hash(first_name), hash(surname));
}

#[test]
fn values_with_different_tags_compare_equal() {
    let first: Tagged<u8, FirstNameTag> = Tagged::new(7);
    let second: Tagged<u8, SurnameTag> = Tagged::new(7);

    assert_eq!(first, second);
}

#[test]
fn values_with_different_tags_compare_in_value_order() {
    let first: Tagged<u8, FirstNameTag> = Tagged::new(7);
    let second: Tagged<u8, SurnameTag> = Tagged::new(8);

    assert!(first < second);
}
