(set-logic ALL)

; Declare Race data type
(declare-datatypes () ((Race Deity Megami Fury Lady)))
; Declare Demon data type
(declare-datatypes () ((Demon (mk-demon (race Race) (id Int) (lv Int) ))))

; Declare inputs constant
(declare-const demon_i Demon)
(declare-const demon_j Demon)

; Declare intermediary constant
(declare-const lv_avg Int)

; Declare output
(declare-const demon_k Demon)

; Define the inputs
(assert (= demon_i (mk-demon Deity 1 38)))
(assert (= demon_j (mk-demon Lady 2 24)))

; Find average level
(assert (= lv_avg (div (+ (lv demon_i) (lv demon_j)) 2 )))

; Race transformation
(assert (=> 
        (and 
            (= (race demon_i) Deity)
            (= (race demon_j) Lady)
        )
        (= (race demon_k) Fury)
    )
)

; Define all demons

; Define the output demon k
(assert (>= (lv demon_k) lv_avg))

; We want the demon with the lowest level
(minimize (lv demon_k))

(check-sat)
(get-model)