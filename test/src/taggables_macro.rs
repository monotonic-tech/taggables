use std::marker::PhantomData;

use taggables::Taggable as _;

taggables::taggables! {
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct TaggedString(String);

    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct WithPublicField(pub u64);

    struct ByteString<const N: usize>([u8; N]);

    struct Matrix<T, const ROWS: usize, const COLS: usize>([[T; COLS]; ROWS],);

    struct WithNamedFields<T, const N: usize,> {
        pub(crate) values: [T; N],
        label: String,
    }

    struct WithTraitBound<K: Ord, V>(std::collections::BTreeMap<K, V>);

    struct WithAssociatedType<T: Iterator, const N: usize> {
        values: [T::Item; N],
    }
}

taggables::semantic_tags! {
    FirstNameTag("FirstName"),
}

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

#[test]
fn const_generic_tuple_retags_without_changing_length() {
    let digest = ByteString::<3>([1, 2, 3], PhantomData);
    let tagged: ByteString<3, FirstNameTag> = digest.retag();

    assert_eq!(tagged.0, [1, 2, 3]);
}

#[test]
fn mixed_generics_preserve_all_arguments() {
    let matrix = Matrix::<String, 1, 2>([["a".into(), "b".into()]], PhantomData);
    let tagged: Matrix<String, 1, 2, FirstNameTag> = matrix.retag();

    assert_eq!(tagged.0, [["a", "b"]]);
}

#[test]
fn named_fields_and_const_generics_compose() {
    let value = WithNamedFields::<String, 2> {
        values: ["a".into(), "b".into()],
        label: "letters".into(),
        __tag: PhantomData,
    };
    let tagged: WithNamedFields<String, 2, FirstNameTag> = value.retag_into();

    assert_eq!(
        (tagged.values, tagged.label),
        (["a".into(), "b".into()], "letters".into())
    );
}

#[test]
fn bounded_type_parameters_retag() {
    let value = WithTraitBound::<String, u8>([("a".into(), 1)].into(), PhantomData);
    let tagged: WithTraitBound<String, u8, FirstNameTag> = value.retag();

    assert_eq!(tagged.0.get("a"), Some(&1));
}

#[test]
fn named_fields_can_use_bounded_associated_types() {
    let value = WithAssociatedType::<std::vec::IntoIter<String>, 1> {
        values: ["Alice".into()],
        __tag: PhantomData,
    };
    let tagged: WithAssociatedType<std::vec::IntoIter<String>, 1, FirstNameTag> = value.retag();

    assert_eq!(tagged.values, ["Alice"]);
}

#[test]
fn const_generic_struct_works_with_semantics() {
    taggables::semantics! { tags for ByteString { Hash<3> } }
    let value: Hash = ByteString([1, 2, 3], PhantomData);

    assert_eq!(value.untag().0, [1, 2, 3]);
}
