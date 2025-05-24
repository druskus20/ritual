use std::{error::Error, fmt::Display};

pub enum HashMapExtError {
    KeyAlreadyExists,
}

impl Display for HashMapExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashMapExtError::KeyAlreadyExists => write!(f, "Key already exists in the HashMap"),
        }
    }
}

impl std::fmt::Debug for HashMapExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HashMapExtError: {self}")
    }
}
impl Error for HashMapExtError {}

pub trait HashmapExt<K, V> {
    fn fallible_insert(&mut self, key: K, value: V) -> Result<&mut V, HashMapExtError>
    where
        K: std::hash::Hash + Eq,
        V: std::fmt::Debug;
}

impl<K, V> HashmapExt<K, V> for std::collections::HashMap<K, V> {
    fn fallible_insert(&mut self, key: K, value: V) -> Result<&mut V, HashMapExtError>
    where
        K: std::hash::Hash + Eq,
        V: std::fmt::Debug,
    {
        if self.contains_key(&key) {
            return Err(HashMapExtError::KeyAlreadyExists);
        }
        Ok(self.entry(key).or_insert(value))
    }
}

impl<K, V> HashmapExt<K, V> for indexmap::IndexMap<K, V> {
    fn fallible_insert(&mut self, key: K, value: V) -> Result<&mut V, HashMapExtError>
    where
        K: std::hash::Hash + Eq,
        V: std::fmt::Debug,
    {
        if self.contains_key(&key) {
            return Err(HashMapExtError::KeyAlreadyExists);
        }
        Ok(self.entry(key).or_insert(value))
    }
}

pub enum ValidationError {
    InvalidValue,
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::InvalidValue => write!(f, "Invalid value provided"),
        }
    }
}
impl std::fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationError: {self}")
    }
}
impl Error for ValidationError {}

pub struct NonEmpty<T>(T);

pub trait Validate
where
    Self: Sized,
{
    type Target;
    fn new_validated(t: Self::Target) -> Result<Self, ValidationError>;
    fn inner(self) -> Self::Target;
}

impl Validate for NonEmpty<String> {
    type Target = String;
    fn new_validated(t: Self::Target) -> Result<Self, ValidationError> {
        if t == "" {
            Err(ValidationError::InvalidValue)
        } else {
            Ok(NonEmpty(t))
        }
    }
    fn inner(self) -> String {
        self.0
    }
}

pub struct NonZero<T>(T);

impl Validate for NonZero<u32> {
    type Target = u32;
    fn new_validated(t: Self::Target) -> Result<Self, ValidationError> {
        if t == 0 {
            Err(ValidationError::InvalidValue)
        } else {
            Ok(NonZero(t))
        }
    }
    fn inner(self) -> u32 {
        self.0
    }
}
