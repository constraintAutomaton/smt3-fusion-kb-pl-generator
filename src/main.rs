// smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
// to describe demons, their fusion, and fusion mechanics for the game
// Shin Megami Tensei III: Nocturne.
// Copyright (C) 2025  Bryan-Elliott Tam
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

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
        license,
    }: CliArg = argh::from_env();

    if license {
        println!("{}", LICENSE);
        return Ok(());
    }

    let demon_knowledge_base =
        create_prolog_demon_knowledge_base(&demon_rdf_file_path, &race_rdf_file_path)?;

    let fusion_rule_knowledge_base =
        create_prolog_fusion_rule_knowledge_base(&fusion_rule_rdf_file_path, &race_rdf_file_path)?;

    let knowledge_base: Vec<String> = [LICENSE_PROLOG_FILE.to_string(), "".to_string()]
        .into_iter()
        .chain([DOC_KNOWLEDGE_BASE.to_string(), "".to_string()])
        .into_iter()
        .chain([DOC_DEMON_RULE.to_string()])
        .chain(demon_knowledge_base)
        .chain(["".to_string()])
        .chain([DOC_FUSION_RULE.to_string()])
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
const LICENSE_PROLOG_FILE: &'static str = r#"% smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
% to describe demons, their fusion, and fusion mechanics for the game 
% Shin Megami Tensei III: Nocturne.
% Copyright (C) 2025  Bryan-Elliott Tam
%
% This program is free software; you can redistribute it and/or modify
% it under the terms of the GNU General Public License as published by
% the Free Software Foundation; either version 2 of the License, or
% (at your option) any later version.
%
% This program is distributed in the hope that it will be useful,
% but WITHOUT ANY WARRANTY; without even the implied warranty of
% MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
% GNU General Public License for more details.
%
% You should have received a copy of the GNU General Public License along
% with this program; if not, write to the Free Software Foundation, Inc.,
% 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
"#;
const LICENSE: &'static str = r#"smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
to describe demons, their fusion, and fusion mechanics for the game 
Shin Megami Tensei III: Nocturne.
Copyright (C) 2025  Bryan-Elliott Tam

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA."#;
