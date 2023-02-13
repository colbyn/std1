# WIP

Nothing to see here, just initialized the location of my rust std add-ons

Mostly just wanted a place to factor out my generic associated type based trait utils, e.g.
```rust
pub trait ToRef {
    /// The dedicated type with references therein.
    type Type<'a>: Clone where Self: 'a;
    fn to_ref<'a>(&'a self) -> Self::Type<'a>;
}
```

## Long-term
Simple -unofficial- additions to the rust standard library, via a quickly typeable name. 

