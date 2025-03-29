use crate::error::*;
use crate::util::*;
use anyhow::Error;
use oxigraph::sparql::QuerySolution;
use std::path::PathBuf;

pub fn create_prolog_fusion_rule_knowledge_base(
    rule_rdf_file: &PathBuf,
    race_file_path: &PathBuf,
) -> Result<Vec<String>, Error> {
    let prolog_knowledge_base = create_prolog_knowledge_base(
        vec![rule_rdf_file, race_file_path],
        GET_ALL_FUSION_RULE,
        generate_a_prolog_fact,
    )?;

    Ok(prolog_knowledge_base)
}

fn generate_a_prolog_fact(solution_map: QuerySolution) -> Result<String, Error> {
    let race1 = solution_map.get("race1");
    let race2 = solution_map.get("race2");
    let race_r = solution_map.get("raceR");
    if race1.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "race1".to_string(),
        }
        .into())
    } else if race2.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "race2".to_string(),
        }
        .into())
    } else if race_r.is_none() {
        Err(ErrorProjectionVariableDoesNotExist {
            variable: "raceR".to_string(),
        }
        .into())
    } else {
        let race1 = literal_string_to_string(race1.unwrap(), "race1")?;
        let race2 = literal_string_to_string(race2.unwrap(), "race2")?;
        let race_r = literal_string_to_string(race_r.unwrap(), "raceR")?;

        Ok(format!("fuse_race('{race1}', '{race2}', '{race_r}')."))
    }
}

const GET_ALL_FUSION_RULE: &'static str = "
PREFIX vocab: <https://constraintautomaton.github.io/smt-nocture-db-to-rdf/vocabulary.ttl#>
PREFIX schema: <https://schema.org/>

SELECT ?race1 ?race2 ?raceR WHERE {
    [] vocab:withRace1 ?race1Iri ;
        vocab:withRace2 ?race2Iri ;
        vocab:fusionRaceResult ?raceRIri .

    ?race1Iri schema:name ?race1 .
    ?race2Iri schema:name ?race2 .
    ?raceRIri schema:name ?raceR .
}";

#[cfg(test)]
mod generate_a_prolog_fact_test {
    use super::*;
    use oxigraph::model::*;

    #[test]
    fn should_return_an_error_given_the_race_1_is_not_a_literal() -> Result<(), Error> {
        let solution_map_name_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race1")?,
                Variable::new("race2")?,
                Variable::new("raceR")?,
            ],
            vec![
                Term::BlankNode(BlankNode::default()).into(),
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map_name_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'race1' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race2_is_not_a_literal() -> Result<(), Error> {
        let solution_map_race_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race1")?,
                Variable::new("race2")?,
                Variable::new("raceR")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::BlankNode(BlankNode::default()).into(),
                Term::Literal(Literal::from("b")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map_race_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'race2' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race_r_is_not_a_literal() -> Result<(), Error> {
        let solution_map_level_wrong: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race1")?,
                Variable::new("race2")?,
                Variable::new("raceR")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::BlankNode(BlankNode::default()).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map_level_wrong);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the value of the variable 'raceR' is not a literal".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race1_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race2")?,
                Variable::new("raceR")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'race1' does not exist in the solution map".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race2_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race1")?,
                Variable::new("raceR")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'race2' does not exist in the solution map".to_string()
        );

        Ok(())
    }

    #[test]
    fn should_return_an_error_given_the_race_r_is_not_in_the_solution_map() -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race1")?,
                Variable::new("race2")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map);
        assert!(res.is_err());

        assert_eq!(
            res.unwrap_err().to_string(),
            "the variable 'raceR' does not exist in the solution map".to_string()
        );

        Ok(())
    }


    #[test]
    fn should_return_a_prolog_fact_given_a_valid_solution_map_with_a_boolean_string_cannot_be_fused_with_basic_rules(
    ) -> Result<(), Error> {
        let solution_map: QuerySolution = QuerySolution::from((
            vec![
                Variable::new("race1")?,
                Variable::new("race2")?,
                Variable::new("raceR")?,
            ],
            vec![
                Term::Literal(Literal::from("a")).into(),
                Term::Literal(Literal::from("b")).into(),
                Term::Literal(Literal::from("c")).into(),
            ],
        ));

        let res = generate_a_prolog_fact(solution_map)?;

        assert_eq!(res, "fuse_race('a', 'b', 'c').".to_string());
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
    fn should_return_an_error_given_a_non_existing_rule_file() {
        let rule_rdf_file = PathBuf::from("./");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let resp = create_prolog_fusion_rule_knowledge_base(&rule_rdf_file, &race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<io::Error>())
    }

    #[test]
    fn should_return_an_error_given_a_non_existing_race_file() {
        let rule_rdf_file = PathBuf::from("./test_files/test_valid_rule.ttl");
        let race_file_path = PathBuf::from("./");

        let resp = create_prolog_fusion_rule_knowledge_base(&rule_rdf_file, &race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<io::Error>())
    }

    #[test]
    fn should_return_an_error_given_an_invalid_rule_file() {
        let rule_rdf_file = PathBuf::from("./test_files/test_invalid_rule.ttl");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let resp = create_prolog_fusion_rule_knowledge_base(&rule_rdf_file, &race_file_path);

        assert!(resp.is_err());
        assert!(resp.unwrap_err().is::<RdfSyntaxError>())
    }

    #[test]
    fn should_return_an_error_given_an_invalid_race_file() {
        let rule_rdf_file = PathBuf::from("./test_files/test_invalid_rule.ttl");
        let race_file_path = PathBuf::from("./test_files/test_inconsistent_race.ttl");

        let resp = create_prolog_fusion_rule_knowledge_base(&rule_rdf_file, &race_file_path);

        assert!(resp.is_err());
    }

    #[test]
    fn should_return_a_prolog_knowledge_base() -> Result<(), Error> {
        let rule_rdf_file = PathBuf::from("./test_files/test_valid_rule.ttl");
        let race_file_path = PathBuf::from("./test_files/test_valid_race.ttl");

        let expected_knowledge_base: HashSet<String> = vec![
            "fuse_race('Beast', 'Deity', 'Avatar').".to_string(),
            "fuse_race('Beast', 'Fury', 'Avatar').".to_string(),
        ]
        .into_iter()
        .collect();

        let resp = create_prolog_fusion_rule_knowledge_base(&rule_rdf_file, &race_file_path)?;

        assert_eq!(
            resp.into_iter().collect::<HashSet<String>>(),
            expected_knowledge_base
        );
        Ok(())
    }
}
