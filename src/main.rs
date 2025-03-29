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

    let knowledge_base: Vec<String> = demon_knowledge_base
        .into_iter()
        .chain(fusion_rule_knowledge_base)
        .collect();

    println!("{}", knowledge_base.join("\n"));
    Ok(())
}
