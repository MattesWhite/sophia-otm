//! Conversion to/from RDF literals.

use sophia_api::term::SimpleIri;

/// The datatype of an RDF literal representing this Rust type.
pub trait DataType {
    /// All possible datatypes the Rust type can represent.
    ///
    /// In most cases this will be only one IRI. However, `enum`s may represent
    /// several classes.
    fn datatypes() -> &'static [SimpleIri<'static>];

    /// The datatype of an RDF literal representing this value.
    ///
    /// # Provided implementation
    ///
    /// The provided implementation returns the first element of
    /// `Self::datatypes()`.
    fn datatype(&self) -> &'static SimpleIri<'static> {
        &Self::datatypes()[0]
    }
}
