use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid string: {0}")]
    InvalidStringError(String),
}

pub trait Validator {
    fn valid(s: &str) -> bool;
}

pub struct Email {}
impl Validator for Email {
    fn valid(s: &str) -> bool {
        s.contains('@')
    }
}

pub struct ApiKey {}
impl Validator for ApiKey {
    fn valid(s: &str) -> bool {
        s.len() == 32
    }
}

pub struct SafeString<T>(String, PhantomData<T>);

impl<T> TryFrom<&str> for SafeString<T>
where
    T: Validator,
{
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if T::valid(s) {
            Ok(Self(s.to_string(), PhantomData))
        } else {
            Err(Error::InvalidStringError("Invalid string".to_string()))
        }
    }
}

impl<T> std::fmt::Display for SafeString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> Debug for SafeString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl<'de, T> Deserialize<'de> for SafeString<T>
where
    T: Validator,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|s| {
            if T::valid(&s) {
                Ok(Self(s, PhantomData))
            } else {
                Err(serde::de::Error::custom("Invalid string"))
            }
        })
    }
}

impl<T> Serialize for SafeString<T>
where
    T: Validator,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_construct_safe_string_1() -> Result<(), Error> {
        let s: SafeString<Email> = SafeString::try_from("x@foo.bar")?;
        assert_eq!(s.to_string(), "x@foo.bar");
        Ok(())
    }

    #[test]
    fn test_construct_safe_string_2() -> Result<(), Error> {
        let s: SafeString<Email> = "x@foo.bar".try_into()?;
        assert_eq!(s.to_string(), "x@foo.bar");
        Ok(())
    }

    #[test]
    fn test_construct_safe_string_3() -> Result<(), Error> {
        let s = SafeString::<Email>::try_from("x@foo.bar")?;
        assert_eq!(s.to_string(), "x@foo.bar");
        Ok(())
    }
}
