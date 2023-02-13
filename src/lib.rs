extern crate either as either_lib;

pub mod either {
    pub use ::either_lib::*;
}

pub trait ToRef {
    /// The dedicated type with references therein.
    type Type<'a>: Clone where Self: 'a;
    fn to_ref<'a>(&'a self) -> Self::Type<'a>;

    /// Semantic alias for `ToRef::to_ref`.
    fn to_own_lendable<'a>(&'a self) -> Self::Type<'a> {self.to_ref()}
}
