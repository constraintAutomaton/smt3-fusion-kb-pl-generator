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
Knowledge Base: Demon Fusion System

This knowledge base models demons from the video game *Shin Megami Tensei: Nocturne* 
and their fusion rules. Each demon has a name, race, base level, 
and an indicator of whether special fusion is required.
*/

/**
 * demon(Name, Race, BaseLv, SpecialFusion)
 *
 * Describes a demon.
 *
 * @param Name          The name of the demon.
 * @param Race          The race or category of the demon.
 * @param BaseLv        The base level of the demon.
 * @param SpecialFusion Whether the demon requires special fusion.
 */
demon('Zouchou', 'Kishin', 27, false).
demon('Zhen', 'Wilder', 6, false).
demon('Yurlungur', 'Snake', 66, false).
demon('Yatagarasu', 'Avatar', 46, false).
demon('Yaksini', 'Femme', 43, false).
demon('Yaka', 'Haunt', 17, false).
demon('Wu Kong', 'Fury', 54, true).
demon('WillO\'Wisp', 'Foul', 1, false).
demon('White Rider', 'Fiend', 52, true).
demon('Vishnu', 'Deity', 93, false).
demon('Virtue', 'Divine', 41, false).
demon('Valkyrie', 'Wargod', 33, true).
demon('Uzume', 'Megami', 18, false).
demon('Uriel', 'Seraph', 73, true).
demon('Unicorn', 'Holy', 21, false).
demon('Trumpeter', 'Fiend', 77, true).
demon('Troll', 'Fairy', 38, false).
demon('Titania', 'Fairy', 57, false).
demon('Titan', 'Jirae', 49, false).
demon('Throne', 'Divine', 64, false).
demon('Thor', 'Kishin', 76, false).
demon('The Harlot', 'Fiend', 69, true).
demon('Taraka', 'Femme', 20, false).
demon('TaoTie', 'Vile', 65, false).
demon('Surt', 'Tyrant', 74, false).
demon('Sui-Ki', 'Brute', 62, false).
demon('Sudama', 'Jirae', 13, false).
demon('Succubus', 'Night', 37, false).
demon('Sparna', 'Beast', 54, false).
demon('Slime', 'Foul', 6, false).
demon('Skadi', 'Lady', 74, true).
demon('Shiva', 'Fury', 95, true).
demon('Shikome', 'Femme', 32, false).
demon('Shikigami', 'Brute', 4, false).
demon('Shiki-Ouji', 'Brute', 54, false).
demon('Shiisaa', 'Holy', 13, false).
demon('Shadow', 'Foul', 52, false).
demon('Setanta', 'Fairy', 43, false).
demon('Senri', 'Holy', 27, true).
demon('Scathach', 'Megami', 64, false).
demon('Sati', 'Megami', 48, false).
demon('Sarutahiko', 'Jirae', 35, false).
demon('Sarasvati', 'Megami', 30, false).
demon('Samael', 'Vile', 73, true).
demon('Saki Mitama', 'Mitama', 35, false).
demon('Sakahagi', 'Foul', 45, true).
demon('Red Rider', 'Fiend', 55, true).
demon('Raphael', 'Seraph', 84, true).
demon('Rangda', 'Femme', 72, false).
demon('Rakshasa', 'Haunt', 63, false).
demon('Raja Naga', 'Snake', 37, true).
demon('Raiju', 'Wilder', 25, false).
demon('Quetzalcoatl', 'Snake', 55, false).
demon('Queen Mab', 'Night', 56, true).
demon('Pyro Jack', 'Fairy', 19, false).
demon('Purski', 'Yoma', 48, false).
demon('Principality', 'Divine', 28, false).
demon('Preta', 'Haunt', 4, false).
demon('Power', 'Divine', 33, false).
demon('Pixie', 'Fairy', 2, false).
demon('Pisaca', 'Haunt', 28, false).
demon('Phantom', 'Foul', 42, false).
demon('Pazuzu', 'Vile', 45, false).
demon('Parvati', 'Lady', 57, true).
demon('Pale Rider', 'Fiend', 63, true).
demon('Ose', 'Fallen', 45, false).
demon('Orthrus', 'Beast', 34, false).
demon('Onkot', 'Yoma', 37, false).
demon('Oni', 'Brute', 25, false).
demon('Ongyo-Ki', 'Brute', 81, true).
demon('Okuninushi', 'Kishin', 39, false).
demon('Odin', 'Deity', 65, false).
demon('Oberon', 'Fairy', 46, false).
demon('Nyx', 'Night', 70, false).
demon('Nue', 'Wilder', 31, false).
demon('Nozuchi', 'Snake', 14, false).
demon('Nigi Mitama', 'Mitama', 29, false).
demon('Nekomata', 'Beast', 18, false).
demon('Naga', 'Snake', 28, false).
demon('Mou-Ryo', 'Foul', 7, false).
demon('Mothman', 'Wilder', 43, false).
demon('Mot', 'Tyrant', 91, false).
demon('Momunofu', 'Brute', 20, false).
demon('Mizuchi', 'Snake', 34, false).
demon('Mithra', 'Deity', 78, false).
demon('Minakata', 'Kishin', 17, false).
demon('Mikazuchi', 'Kishin', 45, false).
demon('Michael', 'Seraph', 90, true).
demon('Metatron', 'Seraph', 95, true).
demon('Matador', 'Fiend', 30, true).
demon('Makami', 'Avatar', 22, true).
demon('Mada', 'Vile', 83, false).
demon('Long', 'Dragon', 44, true).
demon('Loki', 'Tyrant', 52, false).
demon('Loa', 'Night', 53, false).
demon('Lilith', 'Night', 80, true).
demon('Lilim', 'Night', 8, false).
demon('Legion', 'Haunt', 49, false).
demon('Laksmi', 'Megami', 54, false).
demon('Lachesis', 'Femme', 63, false).
demon('Kusi Mitama', 'Mitama', 32, false).
demon('Kushinada', 'Lady', 41, false).
demon('Kurama', 'Genma', 38, true).
demon('Koumoku', 'Kishin', 33, false).
demon('Koppa', 'Yoma', 19, false).
demon('Kodama', 'Jirae', 3, false).
demon('Kin-Ki', 'Brute', 59, false).
demon('Kikuri-Hime', 'Lady', 24, false).
demon('Kelpie', 'Fairy', 26, false).
demon('Karasu', 'Yoma', 28, true).
demon('Kali', 'Lady', 67, false).
demon('Kaiwan', 'Night', 47, false).
demon('Jinn', 'Yoma', 44, false).
demon('Jikoku', 'Kishin', 52, false).
demon('Jack Frost', 'Fairy', 7, false).
demon('Isora', 'Yoma', 14, false).
demon('Inugami', 'Beast', 13, false).
demon('Incubus', 'Night', 25, false).
demon('Ikusa', 'Brute', 44, false).
demon('HuaPo', 'Jirae', 5, false).
demon('Hresvelgr', 'Wilder', 75, false).
demon('Horus', 'Deity', 38, false).
demon('High Pixie', 'Fairy', 10, true).
demon('Hell Biker', 'Fiend', 42, true).
demon('Hanuman', 'Genma', 46, true).
demon('Gurr', 'Raptor', 63, true).
demon('Gui Xian', 'Dragon', 24, true).
demon('Gogmagog', 'Jirae', 55, false).
demon('Girimehkala', 'Vile', 58, true).
demon('Garuda', 'Avian', 63, true).
demon('Ganesha', 'Wargod', 58, true).
demon('Gabriel', 'Seraph', 87, true).
demon('Fuu-Ki', 'Brute', 66, false).
demon('Futomimi', 'Kishin', 63, false).
demon('Forneus', 'Fallen', 20, false).
demon('Fomor', 'Night', 18, false).
demon('Flauros', 'Fallen', 68, false).
demon('Flaemis', 'Element', 20, false).
demon('Feng Huang', 'Holy', 36, false).
demon('Erthys', 'Element', 7, false).
demon('Eligor', 'Fallen', 29, false).
demon('Efreet', 'Yoma', 52, true).
demon('Dominion', 'Divine', 50, false).
demon('Dis', 'Yoma', 23, false).
demon('Dionysus', 'Fury', 44, false).
demon('Decarabia', 'Fallen', 58, false).
demon('Datsue-Ba', 'Femme', 7, false).
demon('Dante', 'Fiend', 80, false).
demon('Dakini', 'Femme', 52, false).
demon('Daisoujou', 'Fiend', 37, true).
demon('Cu Chulainn', 'Genma', 52, true).
demon('Clotho', 'Femme', 58, false).
demon('Choronzon', 'Haunt', 11, false).
demon('Chimera', 'Holy', 55, false).
demon('Chatterskull', 'Haunt', 20, false).
demon('Cerberus', 'Beast', 61, false).
demon('Cai-Zhi', 'Avatar', 26, false).
demon('Blob', 'Foul', 16, false).
demon('Black Rider', 'Fiend', 61, true).
demon('Black Ooze', 'Foul', 28, false).
demon('Black Frost', 'Night', 66, false).
demon('Bishamon', 'Kishin', 72, false).
demon('Bicorn', 'Wilder', 15, false).
demon('Berith', 'Fallen', 37, false).
demon('Beiji-Weng', 'Fury', 61, false).
demon('Beelzebub (Fly)', 'Tyrant', 95, true).
demon('Beelzebub', 'Tyrant', 84, false).
demon('Barong', 'Avatar', 60, false).
demon('Baphomet', 'Vile', 33, false).
demon('Baihu', 'Holy', 43, false).
demon('Badb Catha', 'Beast', 23, false).
demon('Atropos', 'Femme', 67, false).
demon('Atavaka', 'Deity', 47, false).
demon('Archangel', 'Divine', 18, false).
demon('Arahabaki', 'Vile', 30, false).
demon('Ara Mitama', 'Mitama', 25, false).
demon('Aquans', 'Element', 15, false).
demon('Apsaras', 'Yoma', 8, false).
demon('Angel', 'Divine', 11, false).
demon('Amaterasu', 'Deity', 56, true).
demon('Albion', 'Entity', 64, true).
demon('Aeros', 'Element', 11, false).
demon('Aciel', 'Tyrant', 77, true).
demon('Abaddon', 'Tyrant', 69, false).

/**
 * fuse_race(Race1, Race2, Result)
 *
 * Describes the fusion of two races.
 *
 * @param Race1  The first race.
 * @param Race2  The second race.
 * @param Result The resulting race after fusion.
 */
fuse_race('Haunt', 'Night', 'Yoma').
fuse_race('Genma', 'Wilder', 'Yoma').
fuse_race('Genma', 'Vile', 'Yoma').
fuse_race('Genma', 'Tyrant', 'Yoma').
fuse_race('Genma', 'Holy', 'Yoma').
fuse_race('Fiend', 'Genma', 'Yoma').
fuse_race('Fairy', 'Wilder', 'Yoma').
fuse_race('Fairy', 'Snake', 'Yoma').
fuse_race('Fairy', 'Lady', 'Yoma').
fuse_race('Fairy', 'Jirae', 'Yoma').
fuse_race('Fairy', 'Fallen', 'Yoma').
fuse_race('Dragon', 'Raptor', 'Yoma').
fuse_race('Brute', 'Divine', 'Yoma').
fuse_race('Beast', 'Jirae', 'Yoma').
fuse_race('Jirae', 'Tyrant', 'Wilder').
fuse_race('Holy', 'Raptor', 'Wilder').
fuse_race('Fiend', 'Jirae', 'Wilder').
fuse_race('Femme', 'Jirae', 'Wilder').
fuse_race('Femme', 'Foul', 'Wilder').
fuse_race('Fallen', 'Femme', 'Wilder').
fuse_race('Brute', 'Foul', 'Wilder').
fuse_race('Beast', 'Raptor', 'Wilder').
fuse_race('Beast', 'Haunt', 'Wilder').
fuse_race('Beast', 'Foul', 'Wilder').
fuse_race('Avatar', 'Raptor', 'Wilder').
fuse_race('Raptor', 'Wilder', 'Vile').
fuse_race('Night', 'Raptor', 'Vile').
fuse_race('Megami', 'Wilder', 'Vile').
fuse_race('Haunt', 'Raptor', 'Vile').
fuse_race('Haunt', 'Lady', 'Vile').
fuse_race('Haunt', 'Jirae', 'Vile').
fuse_race('Fury', 'Seraph', 'Vile').
fuse_race('Fury', 'Lady', 'Vile').
fuse_race('Foul', 'Raptor', 'Vile').
fuse_race('Foul', 'Lady', 'Vile').
fuse_race('Fallen', 'Fury', 'Vile').
fuse_race('Fallen', 'Foul', 'Vile').
fuse_race('Entity', 'Raptor', 'Vile').
fuse_race('Divine', 'Tyrant', 'Vile').
fuse_race('Divine', 'Kishin', 'Vile').
fuse_race('Divine', 'Fiend', 'Vile').
fuse_race('Divine', 'Fallen', 'Vile').
fuse_race('Deity', 'Night', 'Vile').
fuse_race('Megami', 'Raptor', 'Tyrant').
fuse_race('Kishin', 'Raptor', 'Tyrant').
fuse_race('Fury', 'Vile', 'Tyrant').
fuse_race('Fury', 'Raptor', 'Tyrant').
fuse_race('Deity', 'Raptor', 'Tyrant').
fuse_race('Jirae', 'Kishin', 'Snake').
fuse_race('Foul', 'Yoma', 'Snake').
fuse_race('Fairy', 'Night', 'Snake').
fuse_race('Dragon', 'Vile', 'Snake').
fuse_race('Dragon', 'Holy', 'Snake').
fuse_race('Dragon', 'Foul', 'Snake').
fuse_race('Dragon', 'Fallen', 'Snake').
fuse_race('Dragon', 'Fairy', 'Snake').
fuse_race('Divine', 'Yoma', 'Snake').
fuse_race('Divine', 'Night', 'Snake').
fuse_race('Brute', 'Kishin', 'Snake').
fuse_race('Beast', 'Lady', 'Snake').
fuse_race('Beast', 'Dragon', 'Snake').
fuse_race('Avian', 'Fallen', 'Snake').
fuse_race('Avian', 'Divine', 'Snake').
fuse_race('Avatar', 'Beast', 'Snake').
fuse_race('Tyrant', 'Yoma', 'Night').
fuse_race('Tyrant', 'Wilder', 'Night').
fuse_race('Snake', 'Yoma', 'Night').
fuse_race('Snake', 'Wilder', 'Night').
fuse_race('Lady', 'Yoma', 'Night').
fuse_race('Fiend', 'Yoma', 'Night').
fuse_race('Fiend', 'Wilder', 'Night').
fuse_race('Femme', 'Genma', 'Night').
fuse_race('Fallen', 'Wilder', 'Night').
fuse_race('Fallen', 'Kishin', 'Night').
fuse_race('Fallen', 'Haunt', 'Night').
fuse_race('Fairy', 'Vile', 'Night').
fuse_race('Fairy', 'Tyrant', 'Night').
fuse_race('Fairy', 'Haunt', 'Night').
fuse_race('Fairy', 'Fiend', 'Night').
fuse_race('Dragon', 'Femme', 'Night').
fuse_race('Divine', 'Jirae', 'Night').
fuse_race('Deity', 'Fairy', 'Night').
fuse_race('Brute', 'Fairy', 'Night').
fuse_race('Brute', 'Dragon', 'Night').
fuse_race('Beast', 'Tyrant', 'Night').
fuse_race('Beast', 'Fiend', 'Night').
fuse_race('Beast', 'Fallen', 'Night').
fuse_race('Avian', 'Yoma', 'Night').
fuse_race('Avian', 'Fairy', 'Night').
fuse_race('Seraph', 'Yoma', 'Megami').
fuse_race('Genma', 'Seraph', 'Megami').
fuse_race('Genma', 'Kishin', 'Megami').
fuse_race('Fairy', 'Holy', 'Megami').
fuse_race('Entity', 'Yoma', 'Megami').
fuse_race('Entity', 'Fairy', 'Megami').
fuse_race('Divine', 'Seraph', 'Megami').
fuse_race('Divine', 'Lady', 'Megami').
fuse_race('Divine', 'Genma', 'Megami').
fuse_race('Divine', 'Fairy', 'Megami').
fuse_race('Divine', 'Entity', 'Megami').
fuse_race('Divine', 'Dragon', 'Megami').
fuse_race('Deity', 'Yoma', 'Megami').
fuse_race('Deity', 'Holy', 'Megami').
fuse_race('Deity', 'Genma', 'Megami').
fuse_race('Deity', 'Entity', 'Megami').
fuse_race('Deity', 'Divine', 'Megami').
fuse_race('Avian', 'Seraph', 'Megami').
fuse_race('Avian', 'Raptor', 'Megami').
fuse_race('Avian', 'Genma', 'Megami').
fuse_race('Avian', 'Deity', 'Megami').
fuse_race('Avatar', 'Holy', 'Megami').
fuse_race('Avatar', 'Divine', 'Megami').
fuse_race('Avatar', 'Deity', 'Megami').
fuse_race('Night', 'Vile', 'Lady').
fuse_race('Night', 'Tyrant', 'Lady').
fuse_race('Kishin', 'Megami', 'Lady').
fuse_race('Jirae', 'Megami', 'Lady').
fuse_race('Holy', 'Kishin', 'Lady').
fuse_race('Genma', 'Raptor', 'Lady').
fuse_race('Genma', 'Jirae', 'Lady').
fuse_race('Fury', 'Night', 'Lady').
fuse_race('Fury', 'Kishin', 'Lady').
fuse_race('Fury', 'Genma', 'Lady').
fuse_race('Fiend', 'Night', 'Lady').
fuse_race('Femme', 'Tyrant', 'Lady').
fuse_race('Femme', 'Kishin', 'Lady').
fuse_race('Femme', 'Holy', 'Lady').
fuse_race('Femme', 'Fury', 'Lady').
fuse_race('Femme', 'Fiend', 'Lady').
fuse_race('Fallen', 'Wargod', 'Lady').
fuse_race('Fallen', 'Seraph', 'Lady').
fuse_race('Fallen', 'Genma', 'Lady').
fuse_race('Entity', 'Fury', 'Lady').
fuse_race('Entity', 'Femme', 'Lady').
fuse_race('Dragon', 'Wargod', 'Lady').
fuse_race('Dragon', 'Snake', 'Lady').
fuse_race('Dragon', 'Entity', 'Lady').
fuse_race('Deity', 'Femme', 'Lady').
fuse_race('Brute', 'Fury', 'Lady').
fuse_race('Avian', 'Kishin', 'Lady').
fuse_race('Avian', 'Holy', 'Lady').
fuse_race('Avatar', 'Snake', 'Lady').
fuse_race('Vile', 'Wargod', 'Kishin').
fuse_race('Snake', 'Wargod', 'Kishin').
fuse_race('Snake', 'Vile', 'Kishin').
fuse_race('Seraph', 'Wargod', 'Kishin').
fuse_race('Megami', 'Yoma', 'Kishin').
fuse_race('Lady', 'Wargod', 'Kishin').
fuse_race('Lady', 'Raptor', 'Kishin').
fuse_race('Lady', 'Night', 'Kishin').
fuse_race('Jirae', 'Wargod', 'Kishin').
fuse_race('Holy', 'Wargod', 'Kishin').
fuse_race('Holy', 'Snake', 'Kishin').
fuse_race('Fury', 'Snake', 'Kishin').
fuse_race('Fury', 'Holy', 'Kishin').
fuse_race('Femme', 'Snake', 'Kishin').
fuse_race('Femme', 'Lady', 'Kishin').
fuse_race('Entity', 'Holy', 'Kishin').
fuse_race('Entity', 'Fallen', 'Kishin').
fuse_race('Dragon', 'Jirae', 'Kishin').
fuse_race('Deity', 'Wargod', 'Kishin').
fuse_race('Deity', 'Snake', 'Kishin').
fuse_race('Brute', 'Night', 'Kishin').
fuse_race('Brute', 'Deity', 'Kishin').
fuse_race('Avian', 'Wargod', 'Kishin').
fuse_race('Avian', 'Snake', 'Kishin').
fuse_race('Avian', 'Jirae', 'Kishin').
fuse_race('Avian', 'Fury', 'Kishin').
fuse_race('Avian', 'Brute', 'Kishin').
fuse_race('Avatar', 'Jirae', 'Kishin').
fuse_race('Avatar', 'Genma', 'Kishin').
fuse_race('Avatar', 'Femme', 'Kishin').
fuse_race('Avatar', 'Brute', 'Kishin').
fuse_race('Vile', 'Yoma', 'Jirae').
fuse_race('Haunt', 'Yoma', 'Jirae').
fuse_race('Haunt', 'Wilder', 'Jirae').
fuse_race('Femme', 'Night', 'Jirae').
fuse_race('Fallen', 'Yoma', 'Jirae').
fuse_race('Divine', 'Haunt', 'Jirae').
fuse_race('Brute', 'Fallen', 'Jirae').
fuse_race('Beast', 'Wilder', 'Jirae').
fuse_race('Genma', 'Wargod', 'Holy').
fuse_race('Genma', 'Night', 'Holy').
fuse_race('Fury', 'Yoma', 'Holy').
fuse_race('Fairy', 'Yoma', 'Holy').
fuse_race('Fairy', 'Seraph', 'Holy').
fuse_race('Dragon', 'Seraph', 'Holy').
fuse_race('Dragon', 'Genma', 'Holy').
fuse_race('Divine', 'Wargod', 'Holy').
fuse_race('Divine', 'Megami', 'Holy').
fuse_race('Beast', 'Wargod', 'Holy').
fuse_race('Beast', 'Megami', 'Holy').
fuse_race('Beast', 'Kishin', 'Holy').
fuse_race('Beast', 'Entity', 'Holy').
fuse_race('Beast', 'Divine', 'Holy').
fuse_race('Avatar', 'Night', 'Holy').
fuse_race('Avatar', 'Kishin', 'Holy').
fuse_race('Avatar', 'Fury', 'Holy').
fuse_race('Avatar', 'Avian', 'Holy').
fuse_race('Raptor', 'Yoma', 'Haunt').
fuse_race('Lady', 'Wilder', 'Haunt').
fuse_race('Jirae', 'Vile', 'Haunt').
fuse_race('Foul', 'Vile', 'Haunt').
fuse_race('Foul', 'Tyrant', 'Haunt').
fuse_race('Fiend', 'Foul', 'Haunt').
fuse_race('Fallen', 'Night', 'Haunt').
fuse_race('Fairy', 'Raptor', 'Haunt').
fuse_race('Fairy', 'Foul', 'Haunt').
fuse_race('Fairy', 'Femme', 'Haunt').
fuse_race('Brute', 'Vile', 'Haunt').
fuse_race('Brute', 'Tyrant', 'Haunt').
fuse_race('Brute', 'Fiend', 'Haunt').
fuse_race('Tyrant', 'Vile', 'Fury').
fuse_race('Raptor', 'Vile', 'Fury').
fuse_race('Raptor', 'Tyrant', 'Fury').
fuse_race('Megami', 'Vile', 'Fury').
fuse_race('Lady', 'Megami', 'Fury').
fuse_race('Kishin', 'Wargod', 'Fury').
fuse_race('Kishin', 'Lady', 'Fury').
fuse_race('Fiend', 'Vile', 'Fury').
fuse_race('Fiend', 'Raptor', 'Fury').
fuse_race('Fallen', 'Tyrant', 'Fury').
fuse_race('Fallen', 'Lady', 'Fury').
fuse_race('Fallen', 'Fiend', 'Fury').
fuse_race('Entity', 'Wargod', 'Fury').
fuse_race('Entity', 'Snake', 'Fury').
fuse_race('Entity', 'Lady', 'Fury').
fuse_race('Entity', 'Kishin', 'Fury').
fuse_race('Entity', 'Jirae', 'Fury').
fuse_race('Entity', 'Genma', 'Fury').
fuse_race('Dragon', 'Kishin', 'Fury').
fuse_race('Deity', 'Kishin', 'Fury').
fuse_race('Deity', 'Fallen', 'Fury').
fuse_race('Brute', 'Raptor', 'Fury').
fuse_race('Brute', 'Lady', 'Fury').
fuse_race('Brute', 'Entity', 'Fury').
fuse_race('Avian', 'Dragon', 'Fury').
fuse_race('Avatar', 'Lady', 'Fury').
fuse_race('Avatar', 'Entity', 'Fury').
fuse_race('Avatar', 'Dragon', 'Fury').
fuse_race('Vile', 'Wilder', 'Foul').
fuse_race('Raptor', 'Snake', 'Foul').
fuse_race('Jirae', 'Raptor', 'Foul').
fuse_race('Jirae', 'Night', 'Foul').
fuse_race('Haunt', 'Vile', 'Foul').
fuse_race('Haunt', 'Tyrant', 'Foul').
fuse_race('Fiend', 'Haunt', 'Foul').
fuse_race('Femme', 'Raptor', 'Foul').
fuse_race('Femme', 'Haunt', 'Foul').
fuse_race('Fallen', 'Raptor', 'Foul').
fuse_race('Divine', 'Raptor', 'Foul').
fuse_race('Brute', 'Haunt', 'Foul').
fuse_race('Beast', 'Vile', 'Foul').
fuse_race('Beast', 'Femme', 'Foul').
fuse_race('Lady', 'Snake', 'Femme').
fuse_race('Kishin', 'Yoma', 'Femme').
fuse_race('Kishin', 'Snake', 'Femme').
fuse_race('Kishin', 'Night', 'Femme').
fuse_race('Genma', 'Snake', 'Femme').
fuse_race('Genma', 'Lady', 'Femme').
fuse_race('Fury', 'Jirae', 'Femme').
fuse_race('Foul', 'Jirae', 'Femme').
fuse_race('Dragon', 'Night', 'Femme').
fuse_race('Brute', 'Yoma', 'Femme').
fuse_race('Brute', 'Megami', 'Femme').
fuse_race('Brute', 'Holy', 'Femme').
fuse_race('Beast', 'Brute', 'Femme').
fuse_race('Avian', 'Night', 'Femme').
fuse_race('Avian', 'Beast', 'Femme').
fuse_race('Seraph', 'Tyrant', 'Fallen').
fuse_race('Night', 'Snake', 'Fallen').
fuse_race('Night', 'Seraph', 'Fallen').
fuse_race('Megami', 'Night', 'Fallen').
fuse_race('Jirae', 'Snake', 'Fallen').
fuse_race('Haunt', 'Seraph', 'Fallen').
fuse_race('Foul', 'Snake', 'Fallen').
fuse_race('Foul', 'Seraph', 'Fallen').
fuse_race('Fiend', 'Seraph', 'Fallen').
fuse_race('Femme', 'Wilder', 'Fallen').
fuse_race('Fairy', 'Megami', 'Fallen').
fuse_race('Divine', 'Wilder', 'Fallen').
fuse_race('Divine', 'Vile', 'Fallen').
fuse_race('Beast', 'Yoma', 'Fallen').
fuse_race('Megami', 'Snake', 'Fairy').
fuse_race('Holy', 'Night', 'Fairy').
fuse_race('Femme', 'Megami', 'Fairy').
fuse_race('Divine', 'Snake', 'Fairy').
fuse_race('Divine', 'Holy', 'Fairy').
fuse_race('Divine', 'Foul', 'Fairy').
fuse_race('Brute', 'Wilder', 'Fairy').
fuse_race('Brute', 'Jirae', 'Fairy').
fuse_race('Beast', 'Night', 'Fairy').
fuse_race('Beast', 'Genma', 'Fairy').
fuse_race('Seraph', 'Vile', 'Divine').
fuse_race('Night', 'Yoma', 'Divine').
fuse_race('Kishin', 'Seraph', 'Divine').
fuse_race('Holy', 'Yoma', 'Divine').
fuse_race('Holy', 'Seraph', 'Divine').
fuse_race('Holy', 'Megami', 'Divine').
fuse_race('Genma', 'Megami', 'Divine').
fuse_race('Fallen', 'Megami', 'Divine').
fuse_race('Brute', 'Genma', 'Divine').
fuse_race('Beast', 'Fairy', 'Divine').
fuse_race('Avatar', 'Yoma', 'Divine').
fuse_race('Avatar', 'Fallen', 'Divine').
fuse_race('Avatar', 'Fairy', 'Divine').
fuse_race('Megami', 'Wargod', 'Deity').
fuse_race('Megami', 'Seraph', 'Deity').
fuse_race('Lady', 'Seraph', 'Deity').
fuse_race('Fury', 'Wargod', 'Deity').
fuse_race('Fury', 'Tyrant', 'Deity').
fuse_race('Fury', 'Megami', 'Deity').
fuse_race('Fiend', 'Fury', 'Deity').
fuse_race('Entity', 'Seraph', 'Deity').
fuse_race('Entity', 'Megami', 'Deity').
fuse_race('Divine', 'Fury', 'Deity').
fuse_race('Avian', 'Megami', 'Deity').
fuse_race('Avian', 'Entity', 'Deity').
fuse_race('Avatar', 'Wargod', 'Deity').
fuse_race('Avatar', 'Vile', 'Deity').
fuse_race('Avatar', 'Seraph', 'Deity').
fuse_race('Avatar', 'Megami', 'Deity').
fuse_race('Snake', 'Tyrant', 'Brute').
fuse_race('Jirae', 'Wilder', 'Brute').
fuse_race('Haunt', 'Snake', 'Brute').
fuse_race('Foul', 'Night', 'Brute').
fuse_race('Foul', 'Haunt', 'Brute').
fuse_race('Fiend', 'Snake', 'Brute').
fuse_race('Femme', 'Yoma', 'Brute').
fuse_race('Femme', 'Vile', 'Brute').
fuse_race('Fallen', 'Vile', 'Brute').
fuse_race('Fallen', 'Jirae', 'Brute').
fuse_race('Fairy', 'Kishin', 'Brute').
fuse_race('Fairy', 'Fury', 'Brute').
fuse_race('Entity', 'Wilder', 'Brute').
fuse_race('Entity', 'Night', 'Brute').
fuse_race('Entity', 'Haunt', 'Brute').
fuse_race('Entity', 'Foul', 'Brute').
fuse_race('Deity', 'Jirae', 'Brute').
fuse_race('Beast', 'Snake', 'Brute').
fuse_race('Avian', 'Femme', 'Brute').
fuse_race('Wilder', 'Yoma', 'Beast').
fuse_race('Night', 'Wilder', 'Beast').
fuse_race('Jirae', 'Yoma', 'Beast').
fuse_race('Jirae', 'Lady', 'Beast').
fuse_race('Holy', 'Jirae', 'Beast').
fuse_race('Foul', 'Wilder', 'Beast').
fuse_race('Fallen', 'Snake', 'Beast').
fuse_race('Fallen', 'Holy', 'Beast').
fuse_race('Divine', 'Femme', 'Beast').
fuse_race('Brute', 'Snake', 'Beast').
fuse_race('Brute', 'Femme', 'Beast').
fuse_race('Holy', 'Lady', 'Avatar').
fuse_race('Dragon', 'Yoma', 'Avatar').
fuse_race('Dragon', 'Megami', 'Avatar').
fuse_race('Beast', 'Holy', 'Avatar').
fuse_race('Beast', 'Fury', 'Avatar').
fuse_race('Beast', 'Deity', 'Avatar').
