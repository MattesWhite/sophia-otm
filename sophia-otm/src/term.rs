//! Conversion to/from single terms.
//!
//! For more specific conversion to/from RDF literals see the [`literal`]
//! module.
//!
//! [`literal`]: ../literal/index.html

use sophia_api::term::SimpleIri;

/// Implementing types can be identified as an `rdfs:Class`.
pub trait RdfsClass {
    /// All possible classes the type can represent.
    ///
    /// In most cases this will be only one IRI. However, `enum`s may represent
    /// several classes.
    fn classes() -> &'static [SimpleIri<'static>];

    /// The class of this value.
    ///
    /// # Provided implementation
    ///
    /// The provided implementation returns the first element of
    /// `Self::classes()`.
    fn class(&self) -> &'static SimpleIri<'static> {
        &Self::classes()[0]
    }
}
