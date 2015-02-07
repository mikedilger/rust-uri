
pub mod scheme;
use scheme::Scheme;

#[allow(dead_code)]
/// A data representation for a Uri.  The type S is the scheme data, which may
/// be a String in cases where there is no scheme parser available.
pub struct Uri<S> {
    scheme: Scheme,
    scheme_data: S,
}

/*
impl Uri<S> {
    pub fn parser(data: &str) -> Option<Uri<S>> {
        for c in data {
            match c {
                'a'...'z' | 'A'...'Z' | '0'...'9' | '+' | '-' | '.' => (),
            }
        }
    }
}
*/
