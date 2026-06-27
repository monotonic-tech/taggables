use super::*;

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
