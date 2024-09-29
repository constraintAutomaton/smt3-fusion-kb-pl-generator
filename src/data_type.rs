use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Demon {
    name: String,
    race: Race,
    lv: u8,
    fused_with_basic_rule: bool,
}

#[derive(Debug, Clone)]
pub struct BasicFusionRule {
    result: Race,
    demon1: Race,
    demon2: Race,
}

#[derive(Debug, Clone)]
pub enum Race {
    Wargod,
    Raptor,
    Fairy,
    Deity,
    Wilder,
    Avian,
    Femme,
    Jirae,
    Holy,
    Haunt,
    Entity,
    Fiend,
    Tyrant,
    Divine,
    Night,
    Element,
    Avatar,
    Kishin,
    Megami,
    Vile,
    Fallen,
    Seraph,
    Beast,
    Fury,
    Brute,
    Lady,
    Yoma,
    Genma,
    Snake,
    Foul,
    Dragon,
    Mitama,
}


impl Display for Race{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let race: &str = match self {
            Race::Wargod => "Wargod",
            Race::Raptor => "Raptor",
            Race::Fairy=> "Fairy",
            Race::Deity =>"Deity" ,
            Race::Wilder  => "Wilder",
            Race::Avian => "Avian" ,
            Race::Femme =>"Femme",
            Race::Jirae => "Jirae",
            Race::Holy => "Holy",
            Race::Haunt =>"Haunt",
            Race::Entity => "Entity",
            Race::Fiend => "Fiend",
            Race::Tyrant => "Tyrant",
            Race::Divine => "Divine",
            Race::Night =>"Night",
            Race::Element => "Element",
            Race::Avatar => "Avatar",
            Race::Kishin => "Kishin",
            Race::Megami => "Megami",
            Race::Vile => "Vile",
            Race::Fallen => "Fallen",
            Race::Seraph => "Seraph",
            Race::Beast => "Beast",
            Race::Fury => "Fury",
            Race::Brute => "Brute",
            Race::Lady => "Lady",
            Race::Yoma => "Yoma",
            Race::Genma => "Genma",
            Race::Snake => "Snake",
            Race::Foul => "Foul",
            Race::Dragon => "Dragon",
            Race::Mitama => "Mitama",
        };
        write!(f, "{}", race)
    }
}

impl TryFrom<String> for Race {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.as_str();
        Race::try_from(value)
    }
}

impl TryFrom<&str> for Race {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Wargod" => Ok(Race::Wargod),
            "Raptor" => Ok(Race::Raptor),
            "Fairy" => Ok(Race::Fairy),
            "Deity" => Ok(Race::Deity),
            "Wilder" => Ok(Race::Wilder),
            "Avian" => Ok(Race::Avian),
            "Femme" => Ok(Race::Femme),
            "Jirae" => Ok(Race::Jirae),
            "Holy" => Ok(Race::Holy),
            "Haunt" => Ok(Race::Haunt),
            "Entity" => Ok(Race::Entity),
            "Fiend" => Ok(Race::Fiend),
            "Tyrant" => Ok(Race::Tyrant),
            "Divine" => Ok(Race::Divine),
            "Night" => Ok(Race::Night),
            "Element" => Ok(Race::Element),
            "Avatar" => Ok(Race::Avatar),
            "Kishin" => Ok(Race::Kishin),
            "Megami" => Ok(Race::Megami),
            "Vile" => Ok(Race::Vile),
            "Fallen" => Ok(Race::Fallen),
            "Seraph" => Ok(Race::Seraph),
            "Beast" => Ok(Race::Beast),
            "Fury" => Ok(Race::Fury),
            "Brute" => Ok(Race::Brute),
            "Lady" => Ok(Race::Lady),
            "Yoma" => Ok(Race::Yoma),
            "Genma" => Ok(Race::Genma),
            "Snake" => Ok(Race::Snake),
            "Foul" => Ok(Race::Foul),
            "Dragon" => Ok(Race::Dragon),
            "Mitama" => Ok(Race::Mitama),
            _ => Err(format!("{} cannot be converted into a valid Race", value)),
        }
    }
}
