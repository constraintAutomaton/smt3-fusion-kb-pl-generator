mod cli;
mod create_knowledge_base;
mod error;

use crate::cli::*;
use anyhow::Error;
use create_knowledge_base::*;

fn main() -> Result<(), Error> {
    let CliArg {
        demon_rdf_file_path,
        race_rdf_file_path,
    }: CliArg = argh::from_env();
    let knowledge_base = create_prolog_knowledge_base(demon_rdf_file_path, race_rdf_file_path)?;
    println!("{}", knowledge_base.join("\n"));
    Ok(())
}
