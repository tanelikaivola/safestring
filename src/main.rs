mod safestring;
use safestring::{ApiKey, Email, SafeString};

fn main() {
    let s: SafeString<Email> = safestring::SafeString::new("foo@bar.baz");
    let a: SafeString<ApiKey> = safestring::SafeString::new("0123456789abcdef0123456789abcdef");
    println!("{s}");
    println!("{a}");

    // let boom: SafeString<ApiKey> = safestring::SafeString::new("0123456789abcdef");
    // println!("{}", a == s); // also boom
}
