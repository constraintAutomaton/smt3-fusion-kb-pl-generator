use crate::error::*;
use crate::util::*;
use anyhow::Error;
use oxigraph::sparql::QuerySolution;
use std::path::PathBuf;

pub fn create_prolog_demon_knowledge_base(
    demon_file_path: &PathBuf,
    race_file_path: &PathBuf,
) -> Result<Vec<String>, Error> {
    let prolog_knowledge_base = create_prolog_knowledge_base(
        vec![demon_file_path, race_file_path],
        GET_DEMON_QUERY,
        generate_a_prolog_fact,
    )?;
    Ok(prolog_knowledge_base)
}

fn generate_a_prolog_fact(solution_map: QuerySolution) -> Result<String, Error> {
    let name = solution_map.get("name");
    let race = solution_map.get("race");
    let lv = solution_map.get("level");
    let special_fusion = solution_map.get("specialFusion");
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
    } else if special_fusion.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "specialFusion".to_string(),
        }
        .into())
    } else {
        let name = literal_string_to_string(name.unwrap(), "name")?.replace("'", "\\'");
        let race = literal_string_to_string(race.unwrap(), "race")?;
        let lv = literal_string_to_string(lv.unwrap(), "level")?;
        let special_fusion = literal_string_to_string(
            special_fusion.unwrap(),
            "specialFusion",
        )?;

        if special_fusion != "true".to_string()
            && special_fusion != "false".to_string()
        {
            return Err(ErrorSolutionExpectedToBeBoolean {
                variable: "specialFusion",
            }
            .into());
        }

        Ok(format!(
            "demon('{name}', '{race}', {lv}, {special_fusion})."
        ))
    }
}

const GET_DEMON_QUERY: &'static str = "
PREFIX vocab: <https://constraintautomaton.github.io/smt-nocture-db-to-rdf/vocabulary.ttl#>
PREFIX schema: <https://schema.org/>

SELECT ?name ?race ?level ?specialFusion WHERE {
    ?demon schema:name ?name;
        vocab:isOfRace ?raceIri;
        vocab:hasBasedLevel ?level;
        vocab:specialFusion ?specialFusion.
    
    ?raceIri schema:name ?race .
}";

#[cfg(test)]
mod generate_a_prolog_fact_test {
    use super::*;
    use oxigraph::model::*;

    #[test]
    fn should_return_an_error_given_the_name_is_not_a_literal() -> Result<(), Error> {
        let solution_map_name_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("specialFusion")?,
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
                Variable::new("specialFusion")?,
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
                Variable::new("specialFusion")?,
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
    fn should_return_an_error_given_the_special_fusion_is_not_a_literal(
    ) -> Result<(), Error> {
        let solution_map_special_fusion_wrong: QuerySolution =
            QuerySolution::from((
                vec![
                    Variable::new("name")?,
                    Variable::new("race")?,
                    Variable::new("level")?,
                    Variable::new("specialFusion")?,
                ],
                vec![
                    Term::Literal(Literal::from("a")).into(),
                    Term::Literal(Literal::from("b")).into(),
                    Term::Literal(Literal::from("c")).into(),
                    Term::BlankNode(BlankNode::default()).into(),
                ],
            ));

        let res = generate_a_prolog_fact(solution_map_special_fusion_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'specialFusion' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_name_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("specialFusion")?,
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
                Variable::new("specialFusion")?,
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
                Variable::new("specialFusion")?,
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
    fn should_return_an_error_given_the_special_fusion_is_not_in_the_solution_map(
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
            "the variable 'specialFusion' does not exist in the solution map"
                .to_string()
        );

        Ok(())
    }
    #[test]
    fn should_return_an_error_given_special_fusion_is_not_a_boolean(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("specialFusion")?,
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

        assert_eq!(res.unwrap_err().to_string(), "the value of the variable 'specialFusion' is not a boolean or a string boolean".to_string());
        Ok(())
    }

    #[test]
    fn should_return_a_prolog_fact_given_a_valid_solution_map_with_a_boolean_special_fusion(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("specialFusion")?,
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

        assert_eq!(res, "demon('a', 'b', c, false).".to_string());
        Ok(())
    }

    #[test]
    fn should_return_a_prolog_fact_given_a_valid_solution_map_with_a_boolean_string_special_fusion(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("name")?,
                Variable::new("race")?,
                Variable::new("level")?,
                Variable::new("specialFusion")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
                Term::Literal(Literal::from("true")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map)?;

        assert_eq!(res, "demon('a', 'b', c, true).".to_string());
        Ok(())
    }
}

#[cfg(test)]
mod create_prolog_knowledge_base_test {
    use oxrdfio::RdfSyntaxError;

    use super::*;
    use std::collections::HashSet;
    use std::io;

    #[test]
    fn should_return_an_error_given_a_non_existing_demon_file() {
        let demon_file_path = PathBuf::from("./");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let resp = create_prolog_demon_knowledge_base(&demon_file_path, &race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<io::Error>())
    }

    #[test]
    fn should_return_an_error_given_a_non_existing_race_file() {
        let demon_file_path = PathBuf::from("./test_files/test_valid_demon.ttl");
        let race_file_path = PathBuf::from("./");

        let resp = create_prolog_demon_knowledge_base(&demon_file_path, &race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<io::Error>())
    }

    #[test]
    fn should_return_an_error_given_an_invalid_demon_file() {
        let demon_file_path = PathBuf::from("./test_files/test_invalid_demon.ttl");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let resp = create_prolog_demon_knowledge_base(&demon_file_path, &race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<RdfSyntaxError>())
    }

    #[test]
    fn should_return_an_error_given_an_invalid_race_file() {
        let demon_file_path = PathBuf::from("./test_files/test_valid_demon.ttl");
        let race_file_path = PathBuf::from("./test_files/test_inconsistent_race.ttl");

        let resp = create_prolog_demon_knowledge_base(&demon_file_path, &race_file_path);

        assert!(resp.is_err());
    }

    #[test]
    fn should_return_a_prolog_knowledge_base() -> Result<(), Error> {
        let demon_file_path = PathBuf::from("./test_files/test_valid_demon.ttl");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let expected_knowledge_base: HashSet<String> = vec![
            "demon('Abaddon', 'Tyrant', 69, false).".to_string(),
            "demon('Aeros', 'Element', 11, false).".to_string(),
        ]
        .into_iter()
        .collect();

        let resp = create_prolog_demon_knowledge_base(&demon_file_path, &race_file_path)?;

        assert_eq!(
            resp.into_iter().collect::<HashSet<String>>(),
            expected_knowledge_base
        );
        Ok(())
    }
}
