use crate::{SemanticTag, taggables};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

taggables! {
    /// Somehow the wrapper returned.
    ///
    /// Use it for foreign types which can not be made [Taggable].
    ///
    /// # Usage
    ///
    /// ```rust
    /// use taggables::{Tagged, WrapWithTag};
    ///
    /// taggables::semantic_tags! {
    ///     pub EmailTag("Email"),
    /// }
    ///
    /// let email: Tagged<String, EmailTag> =
    ///     "bob@example.com".to_string().wrap_with_tag::<EmailTag>();
    ///
    /// assert_eq!(email.as_ref(), "bob@example.com");
    /// assert_eq!(email.into_inner(), "bob@example.com".to_string());
    /// ```
    ///
    /// [Taggable]: crate::Taggable
    pub struct Tagged<T>(T);
}

impl<T, S> Tagged<T, S> {
    /// Wrap `inner`, tagging it with `S`.
    pub const fn new(inner: T) -> Self {
        Self(inner, PhantomData)
    }

    /// Unwrap the value, discarding the tag.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Extension for wrapping arbitrary types into [Tagged].
pub trait WrapWithTag: Sized {
    /// Wrap `self` into a [Tagged] with semantic `S`.
    fn wrap_with_tag<S>(self) -> Tagged<Self, S> {
        Tagged(self, PhantomData)
    }
}

impl<T> WrapWithTag for T {}

impl<T, S> AsRef<T> for Tagged<T, S> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T, S> AsMut<T> for Tagged<T, S> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Clone, S> Clone for Tagged<T, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T: Copy, S> Copy for Tagged<T, S> {}

impl<T: Default, S> Default for Tagged<T, S> {
    fn default() -> Self {
        Self(T::default(), PhantomData)
    }
}

impl<T: fmt::Debug, S: SemanticTag> fmt::Debug for Tagged<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tagged<{}>({:?})", S::name(), self.0)
    }
}

impl<T: PartialEq, S, OtherTag> PartialEq<Tagged<T, OtherTag>> for Tagged<T, S> {
    fn eq(&self, other: &Tagged<T, OtherTag>) -> bool {
        self.0 == other.0
    }
}

impl<T: Eq, S> Eq for Tagged<T, S> {}

impl<T: PartialOrd, S, OtherTag> PartialOrd<Tagged<T, OtherTag>> for Tagged<T, S> {
    fn partial_cmp(&self, other: &Tagged<T, OtherTag>) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: Ord, S> Ord for Tagged<T, S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Hash, S> Hash for Tagged<T, S> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
