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

/// Bulk defines semantics for one or more taggable types, by generating their tags and
/// type aliases.
///
/// ```
/// use taggables::{Taggable, Tagged};
/// use core::marker::PhantomData;
///
/// taggables::semantics! {
///     text_semantics for Tagged {
///         // type parameters canbe passed in semantic definitions
///         pub Email<String>,
///         pub(crate) Username<String>,
///     }
/// }
/// let email = Email::new(String::from("alice@example.com"));
/// assert_eq!(Email::tag_name(), "Email");
/// assert_eq!(email.into_inner(), "alice@example.com");
///
/// taggables::taggables! {
///     struct ByteString<const L: usize>([u8; L]);
/// }
///
/// taggables::semantics! {
///     hash_tags for ByteString {
///         // const generics are supported
///         pub TxHash<32>
///     }
/// }
/// ```
#[macro_export]
macro_rules! semantics {
    (
        $(#[doc = $module_doc:expr])*
        $module_vis:vis $module:ident for $base:ident { $($entries:tt)* }
    ) => {
        $(#[doc = $module_doc])*
        $module_vis mod $module {
            $crate::semantics!(@entries [tag] $($entries)*);
        }
        $crate::semantics!(@entries [alias $base $module] $($entries)*);
    };
    (@entries $mode:tt) => {};
    (@entries $mode:tt , $($rest:tt)*) => {
        $crate::semantics!(@entries $mode $($rest)*);
    };
    (@entries $mode:tt $(#[$($attr:tt)*])* $vis:vis $name:ident < $($rest:tt)*) => {
        $crate::semantics!(@args $mode [$(#[$($attr)*])* $vis $name] [] $($rest)*);
    };
    (@entries $mode:tt $(#[$($attr:tt)*])* $vis:vis $name:ident $($rest:tt)*) => {
        $crate::semantics!(@emit $mode [$(#[$($attr)*])* $vis $name] []);
        $crate::semantics!(@entries $mode $($rest)*);
    };
    (@args $mode:tt $entry:tt [$($args:tt)*] $arg:literal $(,)? > $($rest:tt)*) => {
        $crate::semantics!(@emit $mode $entry [<$($args)* $arg>]);
        $crate::semantics!(@entries $mode $($rest)*);
    };
    (@args $mode:tt $entry:tt [$($args:tt)*] $arg:literal, $($rest:tt)*) => {
        $crate::semantics!(@args $mode $entry [$($args)* $arg,] $($rest)*);
    };
    (@args $mode:tt $entry:tt [$($args:tt)*] $arg:ty $(,)? > $($rest:tt)*) => {
        $crate::semantics!(@emit $mode $entry [<$($args)* $arg>]);
        $crate::semantics!(@entries $mode $($rest)*);
    };
    (@args $mode:tt $entry:tt [$($args:tt)*] $arg:ty, $($rest:tt)*) => {
        $crate::semantics!(@args $mode $entry [$($args)* $arg,] $($rest)*);
    };
    (@args $mode:tt $entry:tt [$($args:tt)*] > $($rest:tt)*) => {
        $crate::semantics!(@emit $mode $entry [<$($args)*>]);
        $crate::semantics!(@entries $mode $($rest)*);
    };
    (@emit [tag] [$(#[$($attr:tt)*])* $vis:vis $name:ident] $args:tt) => {
        $crate::semantics!(@tag $name [] $(#[$($attr)*])*);
    };
    (@emit [alias $base:ident $module:ident]
        [$(#[$($attr:tt)*])* $vis:vis $name:ident] [$($args:tt)*]) => {
        $(#[$($attr)*])*
        #[doc = ::core::concat!("Tagged alias `", ::core::stringify!($name), "`.")]
        $vis type $name = <$base $($args)* as $crate::Taggable>::This<$module::$name>;
    };
    (@tag $name:ident [$($cfg:tt)*] #[cfg($($condition:tt)*)] $($rest:tt)*) => {
        $crate::semantics!(@tag $name [$($cfg)* #[cfg($($condition)*)]] $($rest)*);
    };
    (@tag $name:ident [$($cfg:tt)*] #[cfg_attr($($condition:tt)*)] $($rest:tt)*) => {
        $crate::semantics!(@tag $name [$($cfg)* #[cfg_attr($($condition)*)]] $($rest)*);
    };
    (@tag $name:ident [$($cfg:tt)*] #[$($attr:tt)*] $($rest:tt)*) => {
        $crate::semantics!(@tag $name [$($cfg)*] $($rest)*);
    };
    (@tag $name:ident [$($cfg:tt)*]) => {
        $($cfg)*
        $crate::semantic_tags! { pub $name }
    };
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
///
/// // const generics are supported
/// taggables::taggables! {
///     pub struct Digest<const L: usize>([u8; L]);
/// }
///
/// // as are trait bounds
/// taggables::taggables! {
///     pub struct Sorted<K: Ord, V>(std::collections::BTreeMap<K, V>);
/// }
///
/// // structs with names reserve `__tag` field for the tag
/// taggables::taggables! {
///     struct Batch<T, const N: usize> {
///         values: [T; N],
///         label: String,
///     }
/// }
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
/// - only one trait bound per type parameter
/// - lifetimes, generic defaults, where-clauses are not supported (yet)
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
    () => {};
    ($(#[$meta:meta])* $vis:vis struct $name:ident < $($rest:tt)*) => {
        $crate::taggables!(@generics [$(#[$meta])* $vis struct $name] [] [] $($rest)*);
    };
    ($(#[$meta:meta])* $vis:vis struct $name:ident $($rest:tt)*) => {
        $crate::taggables!(@fields [$(#[$meta])* $vis struct $name] [] [] $($rest)*);
    };

    // Keep parameter declarations separate from the arguments used in type paths.
    (@generics $head:tt [$($params:tt)*] [$($args:tt)*]
        const $param:ident: $ty:ty, $($rest:tt)*) => {
        $crate::taggables!(@generics $head [$($params)* const $param: $ty,]
            [$($args)* $param,] $($rest)*);
    };
    (@generics $head:tt $params:tt $args:tt
        const $param:ident: $ty:ty > $($rest:tt)*) => {
        $crate::taggables!(@generics $head $params $args const $param: $ty, > $($rest)*);
    };
    (@generics $head:tt [$($params:tt)*] [$($args:tt)*]
        $param:ident $(: $bound:path)?, $($rest:tt)*) => {
        $crate::taggables!(@generics $head [$($params)* $param $(: $bound)?,]
            [$($args)* $param,] $($rest)*);
    };
    (@generics $head:tt $params:tt $args:tt
        $param:ident $(: $bound:path)? > $($rest:tt)*) => {
        $crate::taggables!(@generics $head $params $args $param $(: $bound)?, > $($rest)*);
    };
    (@generics $head:tt $params:tt $args:tt > $($rest:tt)*) => {
        $crate::taggables!(@fields $head $params $args $($rest)*);
    };

    (@fields [$(#[$meta:meta])* $vis:vis struct $name:ident]
        [$($params:tt)*] [$($args:tt)*]
        $(($($fvis:vis $arg:ty),* $(,)?))?; $($rest:tt)*) => {
        $(#[$meta])*
        $vis struct $name<$($params)* Tag = ()>(
            $($($fvis $arg,)*)? ::core::marker::PhantomData<fn(Tag) -> Tag>
        );

        impl<$($params)* Tag> $crate::Taggable for $name<$($args)* Tag> {
            type Tag = Tag;
            type This<NewTag> = $name<$($args)* NewTag>;
            fn retag<NewTag>(self) -> Self::This<NewTag> {
                let value = self;
                $crate::taggables!(@retag $name value [] [$($([$arg])*)?]
                    [a b c d e f g h i j k l m n p q])
            }
        }
        $crate::taggables!($($rest)*);
    };
    (@fields [$(#[$meta:meta])* $vis:vis struct $name:ident]
        [$($params:tt)*] [$($args:tt)*]
        {$($(#[$($fmeta:tt)*])* $fvis:vis $field:ident: $arg:ty),* $(,)?} $($rest:tt)*) => {
        $(#[$meta])*
        $vis struct $name<$($params)* Tag = ()> {
            $($(#[$($fmeta)*])* $fvis $field: $arg,)*
            __tag: ::core::marker::PhantomData<fn(Tag) -> Tag>,
        }

        impl<$($params)* Tag> $crate::Taggable for $name<$($args)* Tag> {
            type Tag = Tag;
            type This<NewTag> = $name<$($args)* NewTag>;
            fn retag<NewTag>(self) -> Self::This<NewTag> {
                $crate::taggables!(@named_retag $name self [] []
                    $($(#[$($fmeta)*])* $field,)*)
            }
        }
        $crate::taggables!($($rest)*);
    };

    (@named_retag $name:ident $value:ident $fields:tt [$($cfg:tt)*]
        #[cfg($($condition:tt)*)] $($rest:tt)*) => {
        $crate::taggables!(@named_retag $name $value $fields
            [$($cfg)* #[cfg($($condition)*)]] $($rest)*)
    };
    (@named_retag $name:ident $value:ident $fields:tt [$($cfg:tt)*]
        #[cfg_attr($condition:meta, $($attrs:tt)*)] $($rest:tt)*) => {
        $crate::taggables!(@named_cfg $name $value $fields [$($cfg)*]
            [$($rest)*] [$condition; $($attrs)*])
    };
    (@named_cfg $name:ident $value:ident $fields:tt $cfg:tt $rest:tt
        [$condition:meta;] $($pending:tt)*) => {
        $crate::taggables!(@named_cfg $name $value $fields $cfg $rest $($pending)*)
    };
    (@named_cfg $name:ident $value:ident $fields:tt [$($cfg:tt)*] $rest:tt
        [$condition:meta; cfg($predicate:meta) $(, $($attrs:tt)*)?] $($pending:tt)*) => {
        $crate::taggables!(@named_cfg $name $value $fields
            [$($cfg)* #[cfg(any(not($condition), $predicate))]] $rest
            [$condition; $($($attrs)*)?] $($pending)*)
    };
    (@named_cfg $name:ident $value:ident $fields:tt $cfg:tt $rest:tt
        [$condition:meta; cfg_attr($nested:meta, $($inner:tt)*) $(, $($attrs:tt)*)?]
        $($pending:tt)*) => {
        $crate::taggables!(@named_cfg $name $value $fields $cfg $rest
            [all($condition, $nested); $($inner)*]
            [$condition; $($($attrs)*)?] $($pending)*)
    };
    (@named_cfg $name:ident $value:ident $fields:tt $cfg:tt $rest:tt
        [$condition:meta; $attr:meta $(, $($attrs:tt)*)?] $($pending:tt)*) => {
        $crate::taggables!(@named_cfg $name $value $fields $cfg $rest
            [$condition; $($($attrs)*)?] $($pending)*)
    };
    (@named_cfg $name:ident $value:ident $fields:tt $cfg:tt [$($rest:tt)*]) => {
        $crate::taggables!(@named_retag $name $value $fields $cfg $($rest)*)
    };
    (@named_retag $name:ident $value:ident $fields:tt $cfg:tt
        #[$($attr:tt)*] $($rest:tt)*) => {
        $crate::taggables!(@named_retag $name $value $fields $cfg $($rest)*)
    };
    (@named_retag $name:ident $value:ident [$($fields:tt)*] [$($cfg:tt)*]
        $field:ident, $($rest:tt)*) => {
        $crate::taggables!(@named_retag $name $value
            [$($fields)* $($cfg)* $field: $value.$field,] [] $($rest)*)
    };
    (@named_retag $name:ident $value:ident [$($fields:tt)*] []) => {
        $name { $($fields)* __tag: ::core::marker::PhantomData }
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
