import Day2

def safeCount (reports: List Report) : Int :=
  reports.map (fun r => if r.isSafe then 1 else 0) |>.sum

def safeCountDampened (reports: List Report) : Int :=
  reports.map (fun r => if r.isSafeDampened then 1 else 0) |>.sum

def main : IO Unit := do
  IO.println s!"Safe count (test): {safeCount (← parseReportsFile "../input-test")}"
  IO.println s!"Safe count (input): {safeCount (← parseReportsFile "../input")}"

  IO.println s!"Dampened safe (test): {safeCountDampened (← parseReportsFile "../input-test")}"
  IO.println s!"Dampened safe (input): {safeCountDampened (← parseReportsFile "../input")}"
