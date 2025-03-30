/*
Knowledge Base: Demon Fusion Planner System

This knowledge base models how to fuse demons and how to plan the fusion of demon from the video game *Shin Megami Tensei: Nocturne*  
*/

:- use_module('demon.pl').
:- use_module(library(clpz)).
:- use_module(library(lists)).

/**
 *
 * Describes that fusion of race is symetric.
 */
race_fusion_symmetry(R1, R2, RR) :- fuse_race(R1, R2, RR); fuse_race(R2, R1, RR).

/**
 * Describes the normal fusion process of two demons.
 * The resulting demon must meet the following conditions:
 * - Its race must be determined by `fuse_race/3`.
 * - Its base level must be at least the average level of the input demons.
 * - It must be the lowest-level demon in the knowledge base that satisfies these conditions.
 */  
normal_fusion(demon(Name1, Race1, Lv1, SpecialFusion1), demon(Name2, Race2, Lv2, SpecialFusion2), demon(NameR, RaceR, LvR, false)) :- 
    % Set the demon 1 and the demon 2
    demon(Name1, Race1, Lv1, SpecialFusion1),
    demon(Name2, Race2, Lv2, SpecialFusion2),
    % find the race of the resulting demon
    race_fusion_symmetry(Race1, Race2, RaceR), 
    % calculate the level of the resulting demon
    AvgLv #= (Lv1 + Lv2) // 2,
    % check if the demon
    min_lv_above_avg(RaceR, AvgLv, LvR),
    % find the resulting demon
    demon(NameR, RaceR, LvR, false).

/**
 * Describe the level `MinLv` of a demon with the following conditions:
 * - Its race must be `Race`.
 * - Its level must be greater or equal than `AvgLv`.
 */
min_lv_above_avg(Race, AvgLv, MinLv):- findall(Lv, (demon(_, Race, Lv, false), Lv #>= AvgLv), S), list_min(S, MinLv).
