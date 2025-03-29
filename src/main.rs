mod cli;
mod demon_knowledge_base;
mod error;
mod fusion_rule_knowledge_base;
mod util;

use crate::cli::*;
use anyhow::Error;
use demon_knowledge_base::create_prolog_demon_knowledge_base;
use fusion_rule_knowledge_base::create_prolog_fusion_rule_knowledge_base;

fn main() -> Result<(), Error> {
    let CliArg {
        demon_rdf_file_path,
        race_rdf_file_path,
        fusion_rule_rdf_file_path,
    }: CliArg = argh::from_env();
    let demon_knowledge_base =
        create_prolog_demon_knowledge_base(&demon_rdf_file_path, &race_rdf_file_path)?;

    let fusion_rule_knowledge_base =
        create_prolog_fusion_rule_knowledge_base(&fusion_rule_rdf_file_path, &race_rdf_file_path)?;

    let knowledge_base: Vec<String> = [DOC_KNOWLEDGE_BASE.to_string(),"".to_string()]
        .into_iter()
        .chain([DOC_DEMON_RULE.to_string(),])
        .chain(demon_knowledge_base)
        .chain(["".to_string()])
        .chain([DOC_FUSION_RULE.to_string(),])
        .chain(fusion_rule_knowledge_base)
        .collect();

    println!("{}", knowledge_base.join("\n"));
    Ok(())
}

const DOC_KNOWLEDGE_BASE: &'static str = r#"/*
Knowledge Base: Demon Fusion System

This knowledge base models demons from the video game *Shin Megami Tensei: Nocturne* 
and their fusion rules. Each demon has a name, race, base level, 
and an indicator of whether special fusion is required.
*/"#;

const DOC_DEMON_RULE: &'static str = r#"/**
 * demon(Name, Race, BaseLv, SpecialFusion)
 *
 * Describes a demon.
 *
 * @param Name          The name of the demon.
 * @param Race          The race or category of the demon.
 * @param BaseLv        The base level of the demon.
 * @param SpecialFusion Whether the demon requires special fusion.
 */"#;

const DOC_FUSION_RULE: &'static str = r#"/**
 * fuse_race(Race1, Race2, Result)
 *
 * Describes the fusion of two races.
 *
 * @param Race1  The first race.
 * @param Race2  The second race.
 * @param Result The resulting race after fusion.
 */"#;
