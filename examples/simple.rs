use eyre::Result;
use serde::{Deserialize, Serialize};
use strictstring::{ApiKey, Email, StrictString, Validator};

struct Fullname {}
impl Validator for Fullname {
    fn valid(s: &str) -> bool {
        s.split_whitespace().count() == 2
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: StrictString<Email>,
    api_key: StrictString<ApiKey>,
    fullname: StrictString<Fullname>,
}

fn print_api_key(api_key: &StrictString<ApiKey>) {
    println!("{api_key}");
}

fn main() -> Result<()> {
    let s: StrictString<Email> = StrictString::try_from("foo@bar.baz")?;
    let a: StrictString<ApiKey> = StrictString::try_from("0123456789abcdef0123456789abcdef")?;
    println!("{s}");
    println!("{a}");

    print_api_key(&a);
    // print_api_key(&s); // mismatched types

    let data = r#"
        {
            "email": "foo@bar.baz",
            "api_key": "0123456789abcdef0123456789abcdef",
            "fullname": "John Doe"
        }
    "#;

    let v: User = serde_json::from_str(data)?;
    println!("{v:?}");

    // let boom: SafeString<ApiKey> = safestring::SafeString::new("0123456789abcdef");
    // println!("{}", a == s); // also boom

    let nyes: Result<StrictString<Fullname>, _> = "John Doe".try_into();
    let nno: Result<StrictString<Fullname>, _> = "JohnNo".try_into();

    println!("{nyes:?} {nno:?}");

    Ok(())
}
