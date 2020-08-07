//! Conversion to/from graphs.

use crate::term::RdfsClass;
use crate::MissingTriple;
use sophia_api::graph::{GTripleSource, Graph};
use sophia_api::ns::rdf;
use sophia_api::term::TTerm;
use sophia_api::triple::{Triple, stream::{StreamResult, SourceError}};

/// Allows to build a value from an RDF graph.
pub trait FromGraph: Sized {
    /// Build a value from the resource `s` in the graph.
    fn from_graph<G: Graph>(s: &dyn TTerm, g: &G) -> StreamResult<Self, G::Error, MissingTriple>;
}

/// Extension trait for graphs.
pub trait GraphExt: Sized + Graph {
    /// Returns all triples describing instances of the class(es, i.e.
    /// [`C::classes()`]).
    ///
    /// [`C::classes()`]: ../term/trait.RdfsClass.html
    fn instances<C: RdfsClass>(&self) -> GTripleSource<'_, Self> {
        let iter = C::classes()
            .iter()
            .map(move |c| self.triples_with_po(&rdf::type_, c))
            .flatten();
        Box::new(iter) as _
    }
    /// Tries to build a value from the given resource.
    fn resource_to_value<FG: FromGraph>(
        &self,
        r: &dyn TTerm,
    ) -> StreamResult<FG, Self::Error, MissingTriple> {
        FG::from_graph(r, self)
    }
    /// Builds values from all instances of the class found in the RDF graph.
    fn instances_to_values<C>(&self) -> Box<dyn Iterator<Item = StreamResult<C, Self::Error, MissingTriple>> + '_>
    where
        C: RdfsClass + FromGraph,
    {
        let iter = C::classes()
            .iter()
            .map(move |c| self.triples_with_po(&rdf::type_, c))
            .flatten()
            .map(move |tri| match tri {
                Ok(tri) => C::from_graph(tri.s().as_dyn(), self),
                Err(e) => Err(SourceError(e)),
            });
        Box::new(iter) as _
    }
}

impl<G> GraphExt for G
where
    G: Graph
{}
