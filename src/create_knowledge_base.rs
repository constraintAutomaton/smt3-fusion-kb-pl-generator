use crate::error::*;
use anyhow::Error;
use oxigraph::io::RdfSyntaxError;
use oxigraph::model::*;
use oxigraph::sparql::{QueryResults, QuerySolution};
use oxigraph::store::Store;
use oxrdfio::{RdfFormat, RdfParser};
use std::fs;
use std::path::PathBuf;

pub fn create_prolog_knowledge_base(
    demon_file_path: PathBuf,
    race_file_path: PathBuf,
) -> Result<Vec<String>, Error> {
    let mut prolog_knowledge_base: Vec<String> = Vec::new();
    let store = Store::new()?;

    let demon_triple_string = fs::read_to_string(demon_file_path)?;
    let race_triple_string = fs::read_to_string(race_file_path)?;

    let triples = format!("{demon_triple_string}\n{race_triple_string}");

    let turtle_parser = RdfParser::from_format(RdfFormat::Turtle);

    let quads: Vec<Result<Quad, RdfSyntaxError>> =
        turtle_parser.for_slice(triples.as_bytes()).collect();

    for quad_restults in quads {
        let quad = quad_restults?;
        let _ = store.insert(&quad)?;
    }

    let query_result = store.query(GET_DEMON_QUERY)?;

    if let QueryResults::Solutions(solution_maps) = query_result {
        for result_solution_map in solution_maps {
            let solution_map = result_solution_map?;
            let prolog_fact = generate_a_prolog_fact(solution_map)?;
            prolog_knowledge_base.push(prolog_fact);
        }
    }

    Ok(prolog_knowledge_base)
}

fn generate_a_prolog_fact(solution_map: QuerySolution) -> Result<String, Error> {
    let name = solution_map.get("name");
    let race = solution_map.get("race");
    let lv = solution_map.get("level");
    let cannot_be_fused_with_basic_rules = solution_map.get("cannotBeFusedWithBasicRules");
    if name.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "name".to_string(),
        }
        .into())
    } else if race.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "race".to_string(),
        }
        .into())
    } else if lv.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "level".to_string(),
        }
        .into())
    } else if cannot_be_fused_with_basic_rules.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "cannotBeFusedWithBasicRules".to_string(),
        }
        .into())
    } else {
        let name = literal_string_to_string(name.unwrap(), "name")?;
        let race = literal_string_to_string(race.unwrap(), "race")?;
        let lv = literal_string_to_string(lv.unwrap(), "level")?;
        let cannot_be_fused_with_basic_rules = literal_string_to_string(
            cannot_be_fused_with_basic_rules.unwrap(),
            "cannotBeFusedWithBasicRules",
        )?;

        if cannot_be_fused_with_basic_rules != "true".to_string()
            && cannot_be_fused_with_basic_rules != "false".to_string()
        {
            return Err(ErrorSolutionExpectedToBeBoolean {
                variable: "cannotBeFusedWithBasicRules",
            }
            .into());
        }

        return Ok(format!(
            "demon({name}, {race}, {lv}, {cannot_be_fused_with_basic_rules})."
        ));
    }
}

fn literal_string_to_string(
    term: &Term,
    variable: &'static str,
) -> Result<String, ErrorSolutionExpectedToBeString> {
    if let Term::Literal(literal) = term {
        Ok(literal.value().to_string().to_lowercase())
    } else {
        Err(ErrorSolutionExpectedToBeString { variable })
    }
}

const GET_DEMON_QUERY: &'static str = "
PREFIX vocab: <https://constraintautomaton.github.io/smt-nocture-db-to-rdf/vocabulary.ttl#>
PREFIX schema: <https://schema.org/>

SELECT ?name ?race ?level ?cannotBeFusedWithBasicRules WHERE {
    ?demon schema:name ?name;
        vocab:isOfRace ?raceIri;
        vocab:hasBasedLevel ?level;
        vocab:cannotBeFusedWithBasicRules ?cannotBeFusedWithBasicRules.
    
    ?raceIri schema:name ?race .
}";

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

#[cfg(test)]
mod generate_a_prolog_fact_test {
    use super::*;

    #[test]
    fn should_return_an_error_given_the_name_is_not_a_literal() -> Result<(), Error> {
        let solution_map_name_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::BlankNode(BlankNode::default()).into(),
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map_name_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'name' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race_is_not_a_literal() -> Result<(), Error> {
        let solution_map_race_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::BlankNode(BlankNode::default()).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map_race_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'race' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_level_is_not_a_literal() -> Result<(), Error> {
        let solution_map_level_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::BlankNode(BlankNode::default()).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map_level_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'level' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_cannot_be_fused_with_basic_rules_is_not_a_literal(
    ) -> Result<(), Error> {
        let solution_map_cannot_be_fused_with_basic_rules_wrong: QuerySolution =
            QuerySolution::from((
                vec![
                    Variable::new("name")?,
                    Variable::new("race")?,
                    Variable::new("level")?,
                    Variable::new("cannotBeFusedWithBasicRules")?,
                ],
                vec![
                    Term::Literal(Literal::from("a")).into(),
                    Term::Literal(Literal::from("b")).into(),
                    Term::Literal(Literal::from("c")).into(),
                    Term::BlankNode(BlankNode::default()).into(),
                ],
            ));

        let res = generate_a_prolog_fact(solution_map_cannot_be_fused_with_basic_rules_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'cannotBeFusedWithBasicRules' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_name_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'name' does not exist in the solution map".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'race' does not exist in the solution map".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_level_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'level' does not exist in the solution map".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_cannot_be_fused_with_basic_rules_is_not_in_the_solution_map(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'cannotBeFusedWithBasicRules' does not exist in the solution map"
                .to_string()
        );

        Ok(())
    }
    #[test]
    fn should_return_an_error_given_cannot_be_fused_with_basic_rules_is_not_a_boolean(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
                Term::Literal(Literal::from("d")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);

        assert!(res.is_err());

        assert_eq!(res.unwrap_err().to_string(), "the value of the variable 'cannotBeFusedWithBasicRules' is not a boolean or a string boolean".to_string());
        Ok(())
    }

    #[test]
    fn should_return_a_prolog_fact_given_a_valid_solution_map_with_a_boolean_cannot_be_fused_with_basic_rules(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
                Term::Literal(Literal::new_typed_literal(
                    "false",
                    NamedNode::new("http://www.w3.org/2001/XMLSchema#boolean")?,
                ))
                .into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map)?;

        assert_eq!(res, "demon(a, b, c, false).".to_string());
        Ok(())
    }

    #[test]
    fn should_return_a_prolog_fact_given_a_valid_solution_map_with_a_boolean_string_cannot_be_fused_with_basic_rules(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("cannotBeFusedWithBasicRules")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
                Term::Literal(Literal::from("true")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map)?;

        assert_eq!(res, "demon(a, b, c, true).".to_string());
        Ok(())
    }
}

#[cfg(test)]
mod create_prolog_knowledge_base_test {
    use super::*;
    use std::collections::HashSet;
    use std::io;

    #[test]
    fn should_return_an_error_given_a_non_existing_demon_file() {
        let demon_file_path = PathBuf::from("./");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let resp = create_prolog_knowledge_base(demon_file_path, race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<io::Error>())
    }

    #[test]
    fn should_return_an_error_given_a_non_existing_race_file() {
        let demon_file_path = PathBuf::from("./test_files/test_valid_demon.ttl");
        let race_file_path = PathBuf::from("./");

        let resp = create_prolog_knowledge_base(demon_file_path, race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<io::Error>())
    }

    #[test]
    fn should_return_an_error_given_an_invalid_demon_file() {
        let demon_file_path = PathBuf::from("./test_files/test_invalid_demon.ttl");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let resp = create_prolog_knowledge_base(demon_file_path, race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<RdfSyntaxError>())
    }

    #[test]
    fn should_return_an_error_given_an_invalid_race_file() {
        let demon_file_path = PathBuf::from("./test_files/test_valid_demon.ttl");
        let race_file_path = PathBuf::from("./test_files/test_inconsistent_race.ttl");

        let resp = create_prolog_knowledge_base(demon_file_path, race_file_path);

        assert!(resp.is_err());
    }

    #[test]
    fn should_return_a_prolog_knowledge_base() -> Result<(), Error> {
        let demon_file_path = PathBuf::from("./test_files/test_valid_demon.ttl");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let expected_knowledge_base: HashSet<String> = vec![
            "demon(abaddon, tyrant, 69, false).".to_string(),
            "demon(aeros, element, 11, false).".to_string(),
        ]
        .into_iter()
        .collect();

        let resp = create_prolog_knowledge_base(demon_file_path, race_file_path)?;

        assert_eq!(
            resp.into_iter().collect::<HashSet<String>>(),
            expected_knowledge_base
        );
        Ok(())
    }
}
