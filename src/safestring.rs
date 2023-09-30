use std::marker::PhantomData;

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
impl<T> SafeString<T>
where
    T: Validator,
{
    pub fn try_from(s: &str) -> Option<Self> {
        if T::valid(s) {
            Some(Self(s.to_string(), PhantomData))
        } else {
            None
        }
    }

    pub fn new(s: &str) -> Self {
        match T::valid(s) {
            true => Self(s.to_string(), PhantomData),
            false => panic!("Invalid string"),
        }
    }
}

impl<T> std::fmt::Display for SafeString<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_safe_string() {
        let s: SafeString<Email> = SafeString::<Email>::new("x");
        assert_eq!(s.0, "x");
    }
}
