# plist-ext

A collection of extension traits for the wonderful `plist` crate.

plist-ext is not on crates.io yet, so you will have to add this git repository as a dependency. This also means that until plist-ext is on crates.io, there may be breaking changes.

Everything in the `plist` crate will be re-exported by this crate, so you can remove your direct dependency on `plist` and instead use `plist_ext`.

## Provided Traits

note: the links will be broken until plist-ext is on crates.io

-   [`DictionaryExt`](https://docs.rs/plist-ext/*/plist_ext/trait.DictionaryExt.html)
-   [`ToBytes`](https://docs.rs/plist-ext/*/plist_ext/trait.ToBytes.html)

## Example

```rs
use plist_ext::{
    Dictionary, // plist_ext will re-export everything in plist
    DictionaryExt,
};

let mut dict = Dictionary::new();
assert!(dict.add("key", "value").is_none());
assert_eq!(dict.add("key", "value2"), Some("value".into()));
```

## Features

`plist-ext` forwards the features you want to `plist`. This way, you can disable `serde` support by disabling default features or use the
`enable_unstable_features_that_may_break_with_minor_version_bumps` feature.

If you do not want `plist-ext` to re-export everything from `plist`, you can enable the `no_re_export` feature.

## Contributing

If you have a suggestion for a method we should add, feel free to create a GitHub issue or PR and we can discuss it.
