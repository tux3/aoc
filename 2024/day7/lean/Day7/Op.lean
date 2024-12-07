import Mathlib.Data.Nat.Log

abbrev Op := Nat → Nat → Nat

def Op.cat (a b: Nat): Nat :=
  a * 10^(1 + Nat.log 10 b) + b

#guard Op.cat 0 1 == 1
#guard Op.cat 2 0 == 20
#guard Op.cat 1 2 == 12
#guard Op.cat 2 10 == 210
#guard Op.cat 12 345 == 12345
