import Std.Data.HashMap

abbrev Op := String -- Lazyness!

inductive Value
| nat: Nat -> Value
| ref: String -> Value
deriving Repr, Inhabited

inductive Gate
| op0 (val: Value) : Gate
| op1 (op: Op) (val: Value) : Gate
| op2 (op: Op) (lval: Value) (rval: Value) : Gate
deriving Repr, Inhabited

structure Netlist where
  gates: Std.HashMap String Gate

instance : ToString Value where
  toString: Value -> String
    | Value.nat n => s!"{n}"
    | Value.ref s => s

instance : ToString Gate where
  toString: Gate -> String
    | Gate.op0 val => s!"{val}"
    | Gate.op1 op val => s!"{op} {val}"
    | Gate.op2 op lval rval => s!"{lval} {op} {rval}"
