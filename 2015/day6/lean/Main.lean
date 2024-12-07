import Day6

def main : IO Unit := do
  let input := (← IO.FS.readFile "../input").trimRight.splitOn "\n" |>.map parseLine
  IO.println s!"Number of lights part 1: {execList input execOp1}"
  IO.println s!"Number of lights part 2: {execList input execOp2}"
