import Day5

def main : IO Unit := do
  IO.println s!"Middle page sums (test): {← sumMiddlePagesForFile "../input-test"}"
  IO.println s!"Middle page sums (input): {← sumMiddlePagesForFile "../input"}"
