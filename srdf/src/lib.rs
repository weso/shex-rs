pub mod bnode;
pub mod lang;
pub mod literal;


pub use iri_s::*;
pub use bnode::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_2_iris() {
        let iri1: IriS = IriS::from_str("http://example.org/iri");
        let iri2 = IriS::from_str("http://example.org/iri");
        assert_eq!(iri1, iri2);
    }
}
