use super::*;

#[test]
fn tag_defaults_to_unit() {
    let untagged = TaggedString("John".into(), PhantomData);

    assert_eq!(untagged, TaggedString::<()>("John".into(), PhantomData));
    assert_eq!(TaggedString::<()>::tag_name(), "()");
}

#[test]
fn public_fields_are_really_public() {
    let with_pub = WithPublicField::<FirstNameTag>(7, PhantomData);

    assert_eq!(with_pub.0, 7);
}
