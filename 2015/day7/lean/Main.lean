import Day7

def evalNetlistFromFile (path: System.FilePath) (wireName: String): IO (Nat × Nat) := do
  let netlist := ← Netlist.parseFile path

  -- Part 1: Initial value of wire a
  let gate := netlist.gates[wireName]!
  let (value1, _cache, _) := Id.run $ (gate.eval Gate.resolveWireRef).run (Std.HashMap.empty, netlist)

  -- Part 2: Modify b to have the value of a
  let netlist2: Netlist := ⟨ netlist.gates.insert "b" (Gate.op0 (Value.nat value1)) ⟩
  let (value2, _cache, _) := Id.run $ (gate.eval Gate.resolveWireRef).run (Std.HashMap.empty, netlist2)

  pure (value1, value2)

def main : IO Unit := do
  IO.println s!"Netlist wire 'a' values: {← evalNetlistFromFile "../input" "a"}"
