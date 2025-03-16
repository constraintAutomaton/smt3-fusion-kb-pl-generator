use rio_turtle::TurtleError;
use sophia_api::source::TripleSource;
use sophia_turtle;
use std::collections::HashMap;

pub fn generate_demon_prolog_kb(raw_turtle_demon: String, raw_turtle_race: String) {
    let demon_knowledge_base = String::new();
    let demons: HashMap<String, ParseDemon> = HashMap::new();

    let mut triple_source = sophia_turtle::parser::turtle::parse_str(raw_turtle_demon.as_str());

    triple_source.for_each_triple(|triple| {});
}

fn produce_race_map(raw_turtle_race: String) -> Result<HashMap<String, String>, TurtleError> {
    let mut race_map: HashMap<String, String> = HashMap::new();
    let mut race_parsing_map: HashMap<String, ParseRace> = HashMap::new();

    let mut triple_source = sophia_turtle::parser::turtle::parse_str(raw_turtle_race.as_str());
    triple_source.for_each_triple(|triple| {
        let subject = triple.subject;
        let predicate = triple.predicate;
        let object = triple.object;
        
        if let rio_api::model::Subject::NamedNode(name_node) = subject {
            let race_parsing_entry = race_parsing_map.get_mut(name_node.iri);

            if let Some(race) = race_parsing_entry {
                race.class_defined = true;
            } else {
                race_parsing_map.insert(
                    name_node.iri.to_string(),
                    ParseRace::default_class_defined(),
                );
            }
        }
    })?;

    Ok(race_map)
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

impl ParseRace {
    pub fn default_class_defined() -> Self {
        let mut race = ParseRace::default();
        race.class_defined = true;
        race
    }
}
