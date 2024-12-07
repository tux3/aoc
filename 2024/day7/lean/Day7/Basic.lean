import Day7.Op

structure Equation where
  target: Nat
  terms: List Nat

namespace Equation

def fromLine (l: String): Equation :=
  let elems := l.splitOn ": "
  let target := elems[0]!.toNat!
  let terms := elems[1]!.splitOn " " |>.map (·.toNat!)
  ⟨ target, terms ⟩

def solvable (eq: Equation) (ops: List Op): Bool :=
  let rec solveRec (cur: Nat) (terms: List Nat) (ops: List Op) : Bool := match terms with
    | [] => cur == eq.target
    | term :: terms => ops.any (fun op =>
        let val := op cur term
        if val <= eq.target then solveRec val terms ops
        else false
      )
  solveRec 0 eq.terms ops

end Equation
