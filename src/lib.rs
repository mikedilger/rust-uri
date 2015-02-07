
pub mod scheme;
use scheme::Scheme;

// FIXME: define a richer Query type
type Query = String;

type Fragment = String;

#[allow(dead_code)]
pub struct URI<S: SchemeData> {
    scheme: Scheme,
    scheme_data: S,
    query: Option<Query>,
    fragment: Option<Fragment>,
}


pub trait SchemeData {
    fn parser(data: String) -> Self;
}
