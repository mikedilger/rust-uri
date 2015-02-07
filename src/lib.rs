
#![feature(core)]

extern crate encoding;

use std::string::CowString;

use encoding::{Encoding,DecoderTrap};
use scheme::Scheme;
use std::str::FromStr;

pub mod scheme;

#[allow(dead_code)]
/// A data representation for a Uri
pub struct Uri {
    scheme: Scheme,
    scheme_data: String,
}

#[derive(PartialEq,Eq,Clone,Debug)]
pub enum UriParseError {
    BadSchemeCharacter,
    MissingSchemeTerminator,
    UnknownEncoding,
    EncodingError(CowString<'static>)
}

impl Uri {

    /// Parse bytes in any known encoding into a Uri
    pub fn parse_encoded(data: &[u8], encoding_whatwg_label: &str) -> Result<Uri, UriParseError> {

        // Get encoding from label
        let enc = match ::encoding::label::encoding_from_whatwg_label(encoding_whatwg_label) {
            Some(e) => e,
            None => return Err(UriParseError::UnknownEncoding),
        };

        // Decode input byte sequence
        let decoded = match enc.decode(data, DecoderTrap::Strict) {
            Ok(s) => s,
            Err(e) => return Err(UriParseError::EncodingError(e)),
        };

        Uri::parse(decoded.as_slice())
    }

    /// Parse a UTF-8 encoded &str into a Uri
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

    let uri =  Uri::parse("https://правительство.рф").unwrap();
    assert!(uri.scheme == Scheme::Https);

    match Uri::parse("in/valid://url") {
        Ok(_) => panic!("Invalid URI has parsed!"),
        Err(e) => assert!(e == UriParseError::BadSchemeCharacter),
    };

    match Uri::parse("incomplete") {
        Ok(_) => panic!("Incomplete URI has parsed!"),
        Err(e) => assert!(e == UriParseError::MissingSchemeTerminator),
    };
}

#[test]
fn test_parse_encoded() {
    let input: Vec<u8> = vec![104, 116, 116, 112, 115, 58, 47, 47, 223,
                              224, 208, 210, 216, 226, 213, 219, 236,
                              225, 226, 210, 222, 46, 224, 228];
    let uri = Uri::parse_encoded(input.as_slice(), "cyrillic").unwrap();
    assert!(uri.scheme == Scheme::Https);
    assert!(uri.scheme_data == "//правительство.рф");
}
