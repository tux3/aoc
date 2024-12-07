import Day7

def sumSolvable (eqs: List Equation) (ops: List Op): Nat :=
  eqs.filter (·.solvable ops) |>.map (·.target) |>.sum

def calibrate (path: System.FilePath): IO (Nat × Nat) := do
  let eqs := (← IO.FS.readFile path).trimRight.splitOn "\n" |>.map Equation.fromLine
  pure (sumSolvable eqs [(·+·), (·*·)], sumSolvable eqs [(·+·), (·*·), Op.cat])

def main : IO Unit := do
  IO.println s!"Test calibration: {← calibrate "../input-test"}"
  IO.println s!"Input calibration: {← calibrate "../input"}"
