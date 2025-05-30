// smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
// to describe demons, their fusion, and fusion mechanics for the game 
// Shin Megami Tensei III: Nocturne.
// Copyright (C) 2025  Bryan-Elliott Tam
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::error::*;
use anyhow::Error;
use oxigraph::io::RdfSyntaxError;
use oxigraph::model::*;
use oxigraph::sparql::QueryResults;
use oxigraph::sparql::QuerySolution;
use oxigraph::store::Store;
use oxrdfio::{RdfFormat, RdfParser};
use std::fs;
use std::io;
use std::path::PathBuf;

/// Perform a query over a set of RDF file.
pub(crate) fn query_files(files: Vec<&PathBuf>, query: &'static str) -> Result<QueryResults, Error> {
    let store = Store::new()?;

    let rdf_strings: Vec<Result<String, io::Error>> = files
        .into_iter()
        .map(|file| fs::read_to_string(file))
        .collect();

    let mut triples = String::new();

    for result_rdf_string in rdf_strings {
        let rdf_strings = result_rdf_string?;
        triples.push_str("\n");
        triples.push_str(&rdf_strings);
    }
    let turtle_parser = RdfParser::from_format(RdfFormat::Turtle);

    let quads: Vec<Result<Quad, RdfSyntaxError>> =
        turtle_parser.for_slice(triples.as_bytes()).collect();

    for quad_restults in quads {
        let quad = quad_restults?;
        let _ = store.insert(&quad)?;
    }

    let query_result = store.query(query)?;
    Ok(query_result)
}

/// Create a prolog knowledge base using RDF files, a query and a function to generate fact using the bingings of the query.
pub (crate) fn create_prolog_knowledge_base(
    files: Vec<&PathBuf>,
    query: &'static str,
    generate_a_prolog_fact: fn(QuerySolution) -> Result<String, Error>,
) -> Result<Vec<String>, Error> {
    let mut prolog_knowledge_base: Vec<String> = Vec::new();

    let query_result = query_files(files, query)?;

    if let QueryResults::Solutions(solution_maps) = query_result {
        for result_solution_map in solution_maps {
            let solution_map = result_solution_map?;
            let prolog_fact = generate_a_prolog_fact(solution_map)?;
            prolog_knowledge_base.push(prolog_fact);
        }
    }

    Ok(prolog_knowledge_base)
}

pub(crate) fn literal_string_to_string(
    term: &Term,
    variable: &'static str,
) -> Result<String, ErrorSolutionExpectedToBeString> {
    if let Term::Literal(literal) = term {
        Ok(literal.value().to_string())
    } else {
        Err(ErrorSolutionExpectedToBeString { variable })
    }
}

#[cfg(test)]
mod literal_string_to_string_test {
    use super::*;

    #[test]
    fn should_return_an_error_if_the_term_is_not_a_literal() -> Result<(), Error> {
        let a_blank_node: BlankNode = BlankNode::default();
        let a_named_node: NamedNode = NamedNode::new("http:example.com")?;
        let terms: Vec<Term> = vec![Term::BlankNode(a_blank_node), Term::NamedNode(a_named_node)];
        for term in terms {
            let res = literal_string_to_string(&term, "boo");
            assert!(res.is_err());
            let error = res.unwrap_err();
            assert_eq!(error.variable, "boo");
        }
        Ok(())
    }

    #[test]
    fn should_return_a_string_of_a_literal() -> Result<(), Error> {
        let a_literal: Literal = Literal::from("bar");

        let res = literal_string_to_string(&Term::Literal(a_literal), "foo")?;

        assert_eq!(res, "bar".to_string());

        Ok(())
    }
}
