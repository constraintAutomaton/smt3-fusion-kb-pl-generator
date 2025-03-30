# SMT3 Demon Fusion Knowledge Base Generator

This project generates a Prolog knowledge base to describe demons, their fusion, and fusion mechanics for the game **Shin Megami Tensei III: Nocturne**.
It processes RDF data (located in [`./dataset/rdf`](./dataset/rdf)) from the [Shin Megami Tensei III: Nocturne RDF Datasets](https://constraintautomaton.github.io/smt-nocturne-db-to-rdf/), which is available under the Open Database License (ODbL) [http://opendatacommons.org/licenses/odbl/1.0/](http://opendatacommons.org/licenses/odbl/1.0/) and the Database Contents License (DBCL) [http://opendatacommons.org/licenses/dbcl/1.0/](http://opendatacommons.org/licenses/dbcl/1.0/).
The project generates a Prolog knowledge base, with the most recent version available at [`./dataset/prolog/demon.pl`](./dataset/prolog/demon.pl). Fusion rules for demons are defined in [`./dataset/prolog/planner.pl`](./dataset/prolog/planner.pl).

**Note**: This project has been tested exclusively with [Scryer Prolog](https://www.scryer.pl/).


## Dependencies
- [Rust 2021](https://www.rust-lang.org/)

## Building
`cargo build -r`

## Generate a Prolog Knowledge Base

```
Usage: smt-nocturne-prolog-kb-generator -d <demon-rdf-file-path> -r <race-rdf-file-path> -f <fusion-rule-rdf-file-path> [--license]

Generate a prolog knowledge base to describe and fuse demon from the game Shin Megami Tensei 3 nocture from RDF turtle files.

Options:
  -d, --demon-rdf-file-path
                    the path of a the demon turtle file
  -r, --race-rdf-file-path
                    the path of a the race turtle file
  -f, --fusion-rule-rdf-file-path
                    the path of a the fusion rule turtle file
  --license         print the license
  --help, help      display usage information
```

### Generate The Knowledge Base of the Repository

To generate the Prolog knowledge base, run the following command:
```zsh
cargo run -r -- -d ./dataset/rdf/demon.ttl -r ./dataset/rdf/race.ttl -f ./dataset/rdf/basic_rules.ttl > ./dataset/prolog/demon.pl
```

This command generates the knowledge base as a string, which can be piped for further processing (as shown in the example above).

Alternatively, you can execute the program directly using the binary located in `./target/release/`

## Model

## Premilinaries

Given all Shin Megami Tensei III: Nocturne demons $D$, including a special demon $d_{\empty}$ representing an invalid or non-existent demon, and all races $R$, including a special race $r_{\empty}$ representing an invalid race.  
Demons have unique names $n$ that serve as their ID.  
Each demon has a base level $lv \in \mathbb{R}_{>0}$.  
Let $s \in \{\text{true}, \text{false}\}$ be a boolean indicating whether the demon must be fused with a special condition, with $s = \text{true}$ meaning the demon requires special fusion, and $s = \text{false}$ meaning it does not.

A demon is defined by the tuple:
$$
d := (r \in R, n, lv, s)
$$

We define the following access functions:

- The function
$$
r(D) \to R
$$
returns the race of a demon.

- The function
$$
lv(D) \to \mathbb{R}_{>0}
$$
returns the base level of a demon.

- The function
$$
s(D) \to \{\text{true}, \text{false}\}
$$
returns whether or not a demon requires special fusion, with $s = \text{true}$ indicating special fusion is required, and $s = \text{false}$ indicating it is not.


## Problem

### Normal Fusion
A normal fusion is defined by:
$$
f_d: (D, D) \to D
$$

It takes two demons $d_i$ and $d_j$ and returns a new demon $d_k$.  
A normal fusion can only be result in a demon that do not require special fusion, i.e., $s = \text{false}$.

A race mapping is defined by:
$$
f_R: (R, R) \to R
$$
It describes the resulting race $r_k$ when given input races $r_i$ and $r_j$.

The set of demons that can be fused via normal fusion is given by:
$$
D_{s = \text{false}} = \{ d \in D \mid s(d) = \text{false} \}
$$

The function $f_d$ is concretely defined as follows:  
From the set of race mappings $F_r$, we first obtain the resulting race $r_k$ of the resulting demon $d_k$.  
Next, we calculate the average level $lv_{avg}$ of the input demons:  
$$ lv_{avg} = \frac{lv_i + lv_j}{2} $$

Next, we find the demon of race $r_k$ with the lowest level such that its level is greater than or equal to $lv_{avg}$:
$$
Dp = \{ d_w \in D_{s = \text{false}} \mid r(d_w) = r_k \land lv(D_w) \geq lv_{avg} \}
$$

Finally, we define $d_r$ as the demon in $Dp$ with the minimum level:
$$
d_r = \arg\min_{d \in d_p} lv(d)
$$

The fusion is unsuccessful and return $d_{\empty}$ if the resulting race $r_r$ is $r_{\empty}$.

### Fusion Planner

Given that a user has $n_d$ demons, where $12$ is the maximum number of demons in a set $D_0$, 
the user wishes to acquire a specific demon $d_o$.

A demon can be acquired by:
- Fusion (normal or special)
- Evolution
- Purchase from the compendium
- Recruitment in the field

The problem can be modeled as a state transition problem, where at each step, a demon is acquired via an action $a_i \in A$ where $A$ is all possible actions (defined later), resulting in a new set of demons $D_i$. At the final step, $D_f$, the demon $d_o$ is in this set.

