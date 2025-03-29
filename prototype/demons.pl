:- use_module(library(clpz)).

demon("Horus", deity, 38) .
demon("Dionysus", fury, 44) .
demon("Beji Weng", fury, 61) .
demon("Minakata", kishin, 17) .

% deity, kishin -> fury
fuse_race(Race1, Race2, RaceR) :- Race1=deity, Race2=kishin, RaceR=fury. 

fuse(demon(Name1, Race1, Lv1), demon(Name2, Race2, Lv2), demon(NameR, RaceR, LvR)) :- 
    % find the input demon
    demon(Name1, Race1, Lv1),
    demon(Name2, Race2, Lv2),
    % find the race of the resulting demon
    fuse_race(Race1, Race2, RaceR), 
    % calculate the level of the resulting demon
    LvAvg #= (Lv1 + Lv2) // 2,
    LvR #> LvAvg,
    % check if the demon
    demon_min_level(RaceR, LvR),
    % find the resulting demon
    demon(NameR, RaceR, LvR).

demon_min_level(RaceR, Lv):- LvMore #> Lv, demon(_, RaceR, LvMore) .

