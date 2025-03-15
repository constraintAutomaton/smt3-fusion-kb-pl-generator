(set-logic ALL)


; Declare Race data type
(declare-datatypes () ((Race Deity Megami Fury Lady Kishin None)))
; Declare Demon data type
(declare-datatypes () ((Demon (mk-demon (race Race) (name String) (lv Int) ))))
;

; Declare inputs constant
(declare-const demon_i Demon)
(declare-const demon_j Demon)

(define-fun basicFusionStatement ((demon_database Demon) (demon_expected Demon) (lv_min Int)) Bool
    (if 
        (and
            (= (race demon_expected) (race demon_database))
            (>= (lv demon_database) lv_min )
        )
        true
        false
    )
)

; Declare intermediary constant
(declare-const lv_avg Int)

; Declare output
(declare-const demon_k Demon)

; Define the inputs
(assert (= demon_i (mk-demon Deity "Horus" 38)))
(assert (= demon_j (mk-demon Kishin "Minakata" 17)))
; Find average level
(assert (= lv_avg (div (+ (lv demon_i) (lv demon_j)) 2 )))

; Race transformation
(assert (=> 
        (and 
            (= (race demon_i) Deity)
            (= (race demon_j) Kishin)
        )
        (= (race demon_k) Fury)
    )
)

; Define all demons
(declare-const demon_horus Demon)
(assert (= demon_horus (mk-demon Deity "Horus" 38)))

(declare-const demon_dionysus Demon)
(assert (= demon_dionysus (mk-demon Fury "Dionysus" 44)))

(declare-const demon_beji_weng Demon)
(assert (= demon_beji_weng (mk-demon Fury "Beji Weng" 61)))

; Search for the valid demon
(
    assert(or
        (=> (basicFusionStatement demon_horus demon_k lv_avg) (= demon_k demon_horus)) 
    )
)

(
    assert(or
        (=> (basicFusionStatement demon_dionysus demon_k lv_avg) (= demon_k demon_dionysus)) 
        (=> (basicFusionStatement demon_beji_weng demon_k lv_avg) (= demon_k demon_beji_weng)) 
    )
)

; Define the output demon k
(assert (>= (lv demon_k) lv_avg))

; Check if the fusion is not valid
(assert (not (= (race demon_k) None)))

; We want the demon with the lowest level
(minimize (lv demon_k))

(check-sat)
;(get-model)
(get-value (demon_k)) 