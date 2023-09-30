mod safestring;
use eyre::Result;
use safestring::{ApiKey, Email, SafeString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: SafeString<Email>,
    api_key: SafeString<ApiKey>,
}

fn main() -> Result<()> {
    let s: SafeString<Email> = safestring::SafeString::new("foo@bar.baz");
    let a: SafeString<ApiKey> = safestring::SafeString::new("0123456789abcdef0123456789abcdef");
    println!("{s}");
    println!("{a}");

    let data = r#"
        {
            "email": "foo@bar.baz",
            "api_key": "0123456789abcdef0123456789abcdef"
        }
    "#;

    let v: User = serde_json::from_str(data)?;
    println!("{v:?}");

    // let boom: SafeString<ApiKey> = safestring::SafeString::new("0123456789abcdef");
    // println!("{}", a == s); // also boom

    Ok(())
}
