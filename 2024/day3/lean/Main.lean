import Day3

def main : IO Unit := do
  IO.println s!"Mul (input-test): {computeMul (← IO.FS.readFile "../input-test") false}"
  IO.println s!"Mul (input): {computeMul (← IO.FS.readFile "../input") false}"

  IO.println s!"Mul with control flow (input-test2): {computeMul (← IO.FS.readFile "../input-test2") true}"
  IO.println s!"Mul with control flow (input): {computeMul (← IO.FS.readFile "../input") true}"
