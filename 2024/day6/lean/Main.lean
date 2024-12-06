import Day6

def main : IO Unit := do
  IO.println s!"Walk results (test): {← (← Map.fromFile "../input-test").walkResults}"
  IO.println s!"Walk results (input): {← (← Map.fromFile "../input").walkResults}"
