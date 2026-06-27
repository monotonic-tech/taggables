/// Defines one or more semantic tag types as uninhabited enums.
///
/// # Usage
///
/// ```rust
/// use taggables::SemanticTag;
///
/// taggables::semantic_tags! {
///     // Private by default
///     Email,
///     pub Username,
///     // Override the name
///     pub PasswordTag("Password123!"),
/// }
///
/// assert_eq!(Email::name(), "Email");
/// assert_eq!(Username::name(), "Username");
/// assert_eq!(PasswordTag::name(), "Password123!");
/// ```
///
/// [SemanticTag]: crate::SemanticTag
#[macro_export]
macro_rules! semantic_tags {
    ($($(#[$meta:meta])* $vis:vis $tag:ident $(($name:literal))?),* $(,)?) => {
        $(
            $(#[$meta])*
            #[doc = ::core::concat!("Semantic tag `", $crate::semantic_tags!(@name $tag $($name)?), "`.")]
            // Derive most common traits so naive derives work for user-defined types.
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            $vis enum $tag {}

            impl $crate::SemanticTag for $tag {
                fn name() -> &'static str {
                    $crate::semantic_tags!(@name $tag $($name)?)
                }
            }
        )*
    };

    (@name $tag:ident $name:literal) => { $name };
    (@name $tag:ident) => { ::core::stringify!($tag) };
}

/// Defines one or more taggable types.
///
/// # Usage
/// ```rust
/// # use std::marker::PhantomData;
/// use taggables::Taggable;
///
/// taggables::taggables! {
///     #[derive(Debug, PartialEq, Eq)]
///     pub struct NonEmptyVec<T>(Vec<T>);
/// }
///
/// impl<T, S> NonEmptyVec<T, S> {
///     pub fn new(inner: Vec<T>) -> Self {
///         if inner.is_empty() {
///             panic!("Expected a non-empty vector.");
///         }
///         Self(inner, PhantomData)
///     }
/// }
///
/// taggables::semantic_tags! {
///     pub EmployeeTag,
///     pub CustomerTag,
/// }
///
/// pub type Employees = NonEmptyVec<String, EmployeeTag>;
/// pub type Customers = NonEmptyVec<String, CustomerTag>;
///
/// // Create a list of employees
/// let employees: Employees =
///     Employees::new(vec!["Bob".into(), "Alice".into()]);
///
/// // Promote them to customers
/// let customers: Customers =
///     employees.retag::<CustomerTag>();
/// ```
///
/// # Deriving traits
///
/// Vanilla Rust derives put bounds on all type parameters, which means the semantic
/// tag needs to implement trait `T` for the taggable type to also be `T`.
/// Tags from [semantic_tags] derive the common traits for this reason, but can't cover everything.
/// To avoid the issue in those cases, either manually implement those traits, or use [derive_more]
/// and [derive_where] crates.
///
/// # Limitations
///
/// Only tuple structs with type parameters are supported for now. Named fields,
/// lifetimes, const generics, where-clauses etc. are planned, but will probably
/// need a derive macro. In the meantime, implement [Taggable] by hand.
///
/// Type declarations passed to the macro must not carry a type parameter called `Tag`
/// or `NewTag`, as they will conflict with the tag parameters used by the macro
/// (apparently macro hygiene doesn't cover type params).
///
/// Also, the struct can hold at most 16 fields and there's no good reason to want more.
///
/// [PhantomData]: core::marker::PhantomData
/// [Taggable]: crate::Taggable
/// [semantic_tags]: crate::semantic_tags
/// [derive_more]: https://crates.io/crates/derive-more
/// [derive_where]: https://crates.io/crates/derive-where
#[macro_export]
macro_rules! taggables {
    (
        $(
            $(#[$meta:meta])*
                $vis:vis struct $name:ident
                $(<$($targ:ident),*>)?
                $(($($fvis:vis $arg:ty),*))?
                ;
        )*
    ) => {
        $(
            $(#[$meta])*
                $vis struct $name
                <$($($targ,)*)? Tag = ()>
                ($($($fvis $arg,)*)? ::core::marker::PhantomData<fn(Tag) -> Tag>);

            impl<$($($targ,)*)? Tag>
                $crate::Taggable for $name
                <$($($targ,)*)? Tag> {
                    type Tag = Tag;
                    type This<NewTag> = $name <$($($targ,)*)? NewTag>;
                    fn retag<NewTag>(self) -> Self::This<NewTag> {
                        let value = self;
                        $crate::taggables!(
                            @retag $name value [] [$($( [$arg] )*)?]
                            // if you need more fields than this, you're a psychopath
                            [a b c d e f g h i j k l m n p q]
                        )
                    }
                }
        )*
    };

    // No way to get self.0, self.1 ... so just zip a bunch of identifiers with the fields and destructure.
    (@retag                       $name:ident $value:ident [$($acc:ident)*    ]   [$f:tt $($rest:tt)*] [$id:ident $($ids:ident)*]) => {
        $crate::taggables!(@retag $name       $value       [$($acc      )* $id]   [      $($rest   )*] [          $($ids      )*])
    };
    (@retag $name:ident $value:ident [$($acc:ident)*] [] [$($ids:ident)*]) => {{
        let $name($($acc,)* _) = $value;
        $name($($acc,)* ::core::marker::PhantomData)
    }};
    (@retag $name:ident $value:ident [$($acc:ident)*] [$($rest:tt)+] []) => {
        ::core::compile_error!("`taggables!` supports at most 16 fields")
    };
}
