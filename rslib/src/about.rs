use itertools::Itertools;
use regex::Regex;

pub fn contributors() -> String {
    const CONTRIBUTORS_TXT: &str = include_str!("../../CONTRIBUTORS");
    let contributor_re = Regex::new(r"\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*([\s\S]+)\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*\*").unwrap();
    let email_re = Regex::new(r" *<.+>").unwrap();

    let email_removed = email_re.replace_all(CONTRIBUTORS_TXT, "").to_string();

    let contributors_captures = contributor_re.captures(email_removed.as_str()).unwrap();
    
    let contributors = (&contributors_captures[1]).trim().split("\n");

    let out = contributors
        .map(|s| s.trim().replace(" ", "&nbsp"))
        .join(", ");
    out
}

#[test]
fn contributors_test() {
    let contributors = contributors();
    
    assert!(0 != contributors.len());
}
