//! Re-export of [`plist::dictionary`]

#[cfg(not(feature = "no_re_export"))]
pub use plist::dictionary::*;

/// This allows us to have DictionaryExt in this file without exposing it in the `dictionary` module
pub(super) mod ext {
    use easy_ext::ext;
    use plist::{Dictionary, Value};

    #[ext(DictionaryExt)]
    pub impl Dictionary {
        /// The same as [`Dictionary::insert()`], but accepts `impl Into` to make it less painful to use when inserting many properties.
        ///
        /// # Examples
        ///
        /// ```rs
        /// use plist_ext::{Dictionary, DictionaryExt};
        ///
        /// let mut dict = Dictionary::new();
        /// assert!(dict.add("key", "value").is_none());
        /// assert_eq!(dict.add("key", "value2"), Some("value".into())); // returns Some with old value
        /// ```
        fn add(&mut self, k: impl Into<String>, v: impl Into<Value>) -> Option<Value> {
            self.insert(k.into(), v.into())
        }

        /// This method will only add the key/value pair if the Dictionary doesn't already have a key/value pair with the same key.
        ///
        /// It will return None if it added the key/value pair. Otherwise, if the Dictionary already contained the key, it will return Some with the current value.
        ///
        /// # Examples
        ///
        /// ```rs
        /// use plist_ext::{Dictionary, DictionaryExt};
        ///
        /// let mut dict = Dictionary::new();
        /// assert!(dict.append("key", "value").is_none());
        /// assert_eq!(dict.append("key", "value2"), Some("value".into())); // returns Some with current value
        /// assert_eq!(dict.get("key"), Some(&("value".into()))); // value for "key" is still "value"
        /// ```
        fn append(&mut self, k: impl Into<String>, v: impl Into<Value>) -> Option<Value> {
            let k = &k.into();
            if let Some(current) = self.get(k) {
                Some(current.to_owned())
            } else {
                self.add(k, v);
                None
            }
        }

        /// Allows you to initialize a Dictionary using an array.
        ///
        /// Unfortunately, you will need to `.into()` for the values.
        ///
        /// # Examples
        ///
        /// ```rs
        /// use plist_ext::{Dictionary, DictionaryExt};
        ///
        /// let mut dict = Dictionary::with([
        ///     ("integer", 1.into()),
        ///     ("string", "value".into()),
        /// ]);
        /// assert_eq!(dict.get("integer"), Some(&(1.into())));
        /// assert_eq!(dict.get("string"), Some(&("value".into())));
        /// ```
        fn with<const N: usize>(entries: [(&str, Value); N]) -> Self {
            let mut dict = Self::new();
            for (key, value) in entries {
                dict.add(key, value);
            }
            dict
        }
    }
}

#[cfg(test)]
mod tests {
    use plist::Dictionary;

    use crate::DictionaryExt;

    #[test]
    fn add() {
        let mut dict = Dictionary::new();
        assert!(dict.add("key", "value").is_none());
        assert_eq!(dict.add("key", "value2"), Some("value".into()));
    }

    #[test]
    fn append() {
        let mut dict = Dictionary::new();
        assert!(dict.append("key", "value").is_none());
        assert_eq!(dict.append("key", "value2"), Some("value".into()));
        assert_eq!(dict.get("key"), Some(&("value".into())));
    }

    #[test]
    fn with() {
        let dict = Dictionary::with([("test1", 1.into()), ("test2", "value".into())]);
        assert_eq!(dict.get("test1"), Some(&(1.into())));
        assert_eq!(dict.get("test2"), Some(&("value".into())));
    }
}
