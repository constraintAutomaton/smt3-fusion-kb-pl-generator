% smt-nocturne-prolog-kb-generator: A generates of a Prolog knowledge base
% to describe demons, their fusion, and fusion mechanics for the game 
% Shin Megami Tensei III: Nocturne.
% Copyright (C) 2025  Bryan-Elliott Tam
%
% This program is free software: you can redistribute it and/or modify
% it under the terms of the GNU General Public License as published by
% the Free Software Foundation, either version 3 of the License, or
% (at your option) any later version.
%
% This program is distributed in the hope that it will be useful,
% but WITHOUT ANY WARRANTY; without even the implied warranty of
% MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
% GNU General Public License for more details.
%
% You should have received a copy of the GNU General Public License
% along with this program.  If not, see <https://www.gnu.org/licenses/>.

/*
Knowledge Base: Demon Fusion Planner System

This knowledge base models how to fuse demons and how to plan the fusion of demon from the video game *Shin Megami Tensei: Nocturne*  
*/

:- use_module('demon.pl').
:- use_module(library(clpz)).
:- use_module(library(lists)).
:- use_module(library(debug)).
:- use_module(library(pairs)).
:- use_module(library(dif)).
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

/**
 * A normal fusion step.
 */
normal_fusion(_, _).

/**
 * Describe a fusion plan, where the desired demon is in the demon set D0.
 * - The demon set is `D0`.
 * - The desired demon is `demon(NameR, RaceR, LvR, _)`.
 */
plan(D0, demon(NameR, RaceR, LvR, _)) --> 
    [],
    { 
        member(demon(NameR, RaceR, LvR, _), D0)
    }.

/**
 * Describe a fusion plan.
 * - The demon set is `D0`.
 * - The desired demon is `demon(NameR, RaceR, LvR, _)`.
 */
plan(D0, demon(NameR, RaceR, LvR, _)) --> 
    [normal_fusion(DemonI, DemonJ)],
    {
        % Make sure there are no duplicate
        list_to_set(D0, D1),
        % two demons have the to be fused
        member(DemonI, D1),
        member(DemonJ, D1),
        dif(DemonI, DemonJ),
        normal_fusion(DemonI, DemonJ, DemonR),
        % the next fusion pool cannot have already fused demons
        select(DemonI, D1, D2),
        select(DemonJ, D2, D3),
        % the next fusion pool have the fused demon
        append(D3, [DemonR], D4) 
    },
    plan(D4, demon(NameR, RaceR, LvR, _)).

/**
 * Describe the shortest fusion plan.
 * - The demon set is `D0`.
 * - The desired demon is `demon(NameR, RaceR, LvR, _)`.
 * - The actions to execute the plan `A`.
 */
shortest_plan(D0, demon(NameR, RaceR, LvR, _), A) :-
    once((
        length(A, _),
        phrase(plan(D0, demon(NameR, RaceR, LvR, _)), A)
    )).