import Day7.Basic

abbrev EvalCache := Std.HashMap String Nat
abbrev EvalStateM := StateM (EvalCache × Netlist)
abbrev Resolver := String → EvalStateM Nat

def Value.eval (v: Value) (resolver: Resolver): EvalStateM Nat := do match v with
  | Value.nat n => pure n
  | Value.ref id => resolver id

def evalOp1 (op: Op) (n: Nat): Nat := match op with
  | "NOT" => (~~~ UInt64.ofNat n).toNat
  | _ => panic! s!"Unknown arity-1 opcode: {op}"

def evalOp2 (op: Op) (l r: Nat): Nat := match op with
  | "OR" => l ||| r
  | "AND" => l &&& r
  | "RSHIFT" => l >>> r
  | "LSHIFT" => l <<< r
  | _ => panic! s!"Unknown arity-2 opcode: {op}"

def Gate.eval (gate: Gate) (resolver: Resolver): EvalStateM Nat := do match gate with
  | Gate.op0 val => val.eval resolver
  | Gate.op1 op val => pure $ evalOp1 op (← val.eval resolver)
  | Gate.op2 op lval rval => pure $ evalOp2 op (← lval.eval resolver) (← rval.eval resolver)

-- Value.eval needs to recurse back into Gate.eval through a resolver,
-- but mutual recursion in Lean is a bit rough for bigger functions,
-- so I'll be passing this partial function around as an argument instead
partial def Gate.resolveWireRef (id: String): EvalStateM Nat := do
  let (cache, netlist) := ← get
  if let some val := cache[id]? then
    pure val
  else
    let val := ← netlist.gates[id]!.eval resolveWireRef
    modify (fun (c, n) => (c.insert id val, n))
    pure val
