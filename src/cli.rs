use argh::FromArgs;
use std::path::PathBuf;

#[derive(FromArgs)]
/// Generate a prolog knowledge base to describe and fuse demon from the game Shin Megami Tensei 3 nocture from RDF turtle files.
pub(crate) struct CliArg {
    /// the path of a the demon turtle file
    #[argh(option, short = 'd')]
    pub demon_rdf_file_path: PathBuf,

    /// the path of a the race turtle file
    #[argh(option, short = 'r')]
    pub race_rdf_file_path: PathBuf,

    /// the path of a the fusion rule turtle file
    #[argh(option, short = 'f')]
    pub fusion_rule_rdf_file_path: PathBuf,
}
