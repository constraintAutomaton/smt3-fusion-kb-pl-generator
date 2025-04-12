// smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
// to describe demons, their fusion, and fusion mechanics for the game 
// Shin Megami Tensei III: Nocturne.
// Copyright (C) 2025  Bryan-Elliott Tam
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


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

    #[argh(switch)]
    /// print the license
    pub license: bool
}
