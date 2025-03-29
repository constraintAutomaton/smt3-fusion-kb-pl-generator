# smt-nocturne-fusion-planner

## Dependencies
- [Rust 2021](https://www.rust-lang.org/)

## Building
`cargo build -r`

## Generate the prolog knowledge base

```
Usage: smt-nocturne-fusion-planner -d <demon-rdf-file-path> -r <race-rdf-file-path>

Generate a prolog knowledge base to describe and fuse demon from the game Shin Megami Tensei 3 nocture from RDF turtle files.

Options:
  -d, --demon-rdf-file-path
                    the path of a the demon turtle file
  -r, --race-rdf-file-path
                    the path of a the race turtle file
  --help, help      display usage information
```

`cargo run -r -- -d ./dataset/rdf/demon.ttl -r ./dataset/rdf/race.ttl -f ./dataset/rdf/basic_rules.ttl > ./dataset/prolog/demon.pl`

it will return the knowledge base as a string it can be piped 

The program can also be executed by directly using the binary located at `./target/release/`

## Model

## Premilinaries

Given all SMT3 demons $D$ and all races $R$.
Demons have unique names $n$ so that they can be considered as an ID.
A demon have a based $lv$.
A demon can also have special conditions related to its fusion.
The special condition can be realted to to getting the demon $si$ from the set of all
the conditions $S_{in}$ (including no special condition $s_{empty}$).
The condition can also related on what what happen when the demon is fused $s_o$ from the pool of all the conditions $S_{out}$ (including no special condition $s_{empty}$).
A demon is defined by the tuple

$$
d := \{r \in R,n, lv\in \mathbb{R}_{>0}, s_{in} \in S_{in}, s_{out} \in S_{out}  \}
$$

## Problem

starting with $j$ $d_i$ demons where $j>0 \land j \leq n_d$ where $n_d=12$ is the maximum number of demon in the party.
We wish to fuse demons to get a specific demon $d_o$ by minimizing the number of fusion operation $n_f$.

## Rules

### Normal fusion
A normal fusion 
$$f_d: (D, D) \to D $$

takes two demons $d_i$ and $d_j$ and returns a new demon $d_k$. 
A normal fusion _can only be performed_ when the demons do not have special conditions when fused $s_{out}$, so when 
$$
s_{i,out} \in d_{i} = s_{empty} \land s_{j, out} \in d_{j} = s_{empty}
$$.
A demon $d_k$ with a special condition to be fused $s_i$ cannot be created with $f_d$, so when 
$$
s_{i, in} \in d_{k} \ne s_{empty}
$$.

Given a mapping $F_R: (R, R) \to R$ between two races $(r_i, r_j)$  to a race $r_k$ and a set of every demons 
$$
D_{s_{in} = s_{empty}} = \{D|s_{w,in}=s_{empty} \}
$$.

In the following lines, we define concretely $f_d$.
From the race mapping, we get the race $r_k$ of the $d_k$, then
we calculate the average level $lv_{avg}$ of the input demon.
Finally, we find the demon of a race $r_k$ with the lowest level where its base level is greater or equal to $lv_{avg}$.
More formally,   
$$
r_k = F_R(r_{i},r_{j}) \\
lv_{avg} = \frac{lv_{i} + lv_{j}}{2} \\

Dp = \{d_w \in D_{s_{in} = s_{empty}} |  r_w = r_k \land lv_w  \geq lv_{avg} \} \\
ifd = \arg \min{\{ lv_{p} \in d_p \in Dp \}} \\

f_d = Dp_{ifd}
$$