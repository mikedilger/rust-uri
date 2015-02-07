
#![feature(core)]

use scheme::Scheme;
use std::str::FromStr;

pub mod scheme;

#[allow(dead_code)]
/// A data representation for a Uri.  The type S is the scheme data, which may
/// be a String in cases where there is no scheme parser available.
pub struct Uri {
    scheme: Scheme,
    scheme_data: String,
}

#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub enum UriParseError {
    BadSchemeCharacter,
    MissingSchemeTerminator,
}

impl Uri {

    pub fn parse(data: &str) -> Result<Uri, UriParseError> {

        let (scheme_str, scheme_data_str) = match data.find(':') {
            Some(i) => (&data[..i], &data[i+1..]),
            None => return Err(UriParseError::MissingSchemeTerminator),
        };
        let scheme: Scheme = match FromStr::from_str(scheme_str) {
            Ok(s) => s,
            Err(_) => return Err(UriParseError::BadSchemeCharacter),
        };

        Ok(Uri {
            scheme: scheme,
            scheme_data: scheme_data_str.to_string(),
        })
    }
}

#[test]
fn test_scheme_parsing() {
    let uri = Uri::parse("http://website.com/page").unwrap();
    assert!(uri.scheme == Scheme::Http);

    let uri = Uri::parse("crazy://idea").unwrap();
    assert!(uri.scheme == Scheme::Unknown("crazy".to_string()));

    match Uri::parse("in/valid://url") {
        Ok(_) => panic!("Invalid URI has parsed!"),
        Err(e) => assert!(e == UriParseError::BadSchemeCharacter),
    };

    match Uri::parse("incomplete") {
        Ok(_) => panic!("Incomplete URI has parsed!"),
        Err(e) => assert!(e == UriParseError::MissingSchemeTerminator),
    };
}
