use anyhow::Error;
use rio_api::model as rio_model;
use rio_turtle::TurtleError;
use sophia_api::source::TripleSource;
use sophia_turtle;
use std::collections::HashMap;

pub fn generate_demon_prolog_kb(
    raw_turtle_demon: String,
    raw_turtle_race: String,
) -> Result<String, Error> {
    let demon_knowledge_base = String::new();
    let demon_parsed_map: HashMap<String, ParseDemon> = HashMap::new();

    let race_map = produce_race_map(raw_turtle_race)?;

    let mut triple_source = sophia_turtle::parser::turtle::parse_str(raw_turtle_demon.as_str());

    triple_source.for_each_triple(|triple| {
        let subject = triple.subject;
        let predicate = triple.predicate;
        let object = triple.object;

        if let rio_model::Subject::NamedNode(name_node) = subject {
            let demon_parsed_entry = demon_parsed_map.get(name_node.iri);
            if predicate.iri == RDF_VOCAB::RDF_TYPE_IRI {
                if let rio_model::Term::NamedNode(class) = object {
                    if class.iri == RDF_VOCAB::RACE_CLASS {
                        if let Some(demon) = demon_parsed_entry {
                            demon.class_defined = true;
                        } else {
                            demon_parsed_map
                                .insert(name_node.iri.to_string(), ParseDemon::default());
                        }
                    }
                }
            } else if predicate.iri == RDF_VOCAB::SCHEMA_NAME_PREDICATE_IRI {
                let demon_name = {
                    if let rio_model::Term::Literal(name) = object {
                        Some(get_value_from_litteral_term(name))
                    } else {
                        None
                    }
                };
            }
        }
    });
}

fn produce_race_map(raw_turtle_race: String) -> Result<HashMap<String, String>, TurtleError> {
    let mut race_parsing_map: HashMap<String, ParseRace> = HashMap::new();

    let mut triple_source = sophia_turtle::parser::turtle::parse_str(raw_turtle_race.as_str());
    triple_source.for_each_triple(|triple| {
        let subject = triple.subject;
        let predicate = triple.predicate;
        let object = triple.object;

        if let rio_model::Subject::NamedNode(name_node) = subject {
            let race_parsing_entry = race_parsing_map.get_mut(name_node.iri);
            if predicate.iri == RDF_VOCAB::RDF_TYPE_IRI {
                if let rio_model::Term::NamedNode(class) = object {
                    if (class.iri == RDF_VOCAB::RACE_CLASS) {
                        if let Some(race) = race_parsing_entry {
                            race.class_defined = true;
                        } else {
                            race_parsing_map
                                .insert(name_node.iri.to_string(), ParseRace::default());
                        }
                    }
                }
            } else if predicate.iri == RDF_VOCAB::SCHEMA_NAME_PREDICATE_IRI {
                let race_name = {
                    if let rio_model::Term::Literal(name) = object {
                        Some(get_value_from_litteral_term(name))
                    } else {
                        None
                    }
                };
                if let Some(race) = race_parsing_entry {
                    race.name = race_name;
                } else {
                    let race = {
                        let mut race = ParseRace::default();
                        race.name = race_name;
                        race
                    };
                    race_parsing_map.insert(name_node.iri.to_string(), race);
                }
            }
        }
    })?;

    Ok(parsed_races_to_race_map(race_parsing_map))
}

fn parsed_races_to_race_map(
    race_parsing_map: HashMap<String, ParseRace>,
) -> HashMap<String, String> {
    let mut race_map: HashMap<String, String> = HashMap::new();

    for (iri, parsed_race) in race_parsing_map.into_iter() {
        if parsed_race.class_defined && parsed_race.name.is_some() {
            race_map.insert(iri, parsed_race.name.unwrap());
        }
    }
    race_map
}

fn get_value_from_litteral_term(literal: rio_model::Literal) -> String {
    match literal {
        rio_model::Literal::LanguageTaggedString { value, language: _ } => value.to_string(),
        rio_model::Literal::Simple { value } => value.to_string(),
        rio_model::Literal::Typed { value, datatype: _ } => value.to_string(),
    }
}

#[derive(Default)]
struct ParseDemon {
    pub name: Option<String>,
    pub race: Option<String>,
    pub level: Option<u8>,
    pub normal_fusion: Option<bool>,
    pub class_defined: bool,
}

#[derive(Default)]
struct ParseRace {
    pub name: Option<String>,
    pub class_defined: bool,
}

mod RDF_VOCAB {
    pub const RDF_TYPE_IRI: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    pub const RACE_CLASS: &'static str =
        "https://constraintautomaton.github.io/smt-nocture-db-to-rdf/vocabulary.ttl#Race";
    pub const SCHEMA_NAME_PREDICATE_IRI: &'static str = "https://schema.org/name";
}
