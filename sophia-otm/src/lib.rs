//! Implementation of triple-object-mapping for [sophia].
//!
//! This crate provides traits and implementations to map from [graph]s to Rust
//! values and back.
//!
//! # Terminology
//!
//! To clarify the used terminology throughout the documentation, following
//! definitions:
//!
//! - _Type_: A _type_ in Rust, e.g. a `struct`, an `enum` or a primitive type
//!   like `i32`.
//! - _Value_: An instance of a _type_.
//! - _Resource_: Something that is identified by an IRI in RDF.
//! - _Class_: A resource that is an `rdfs:Class` or `rdfs:subClassOf` another
//!   _class_.
//! - _Instance_: A resource that is an instance of a _class_.
//!
//! [sophia]: https://crates.io/crates/sophia
//! [graph]: https://docs.rs/sophia_api/0.6.1/sophia_api/graph/trait.Graph.html

#![deny(missing_docs)]

pub mod graph;
pub mod literal;
pub mod term;

mod error;
pub use self::error::MissingTriple;
