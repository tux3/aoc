import Day9

def main : IO Unit := do
  let inputGraph := ← Graph.parseFile "../input"
  IO.println s!"Input graph: \n{inputGraph}"

  -- let cost := path.toList.zip (path.toList.tail!)
  --   |>.map (fun (src, dst) =>
  --     inputGraph[src]!.find? (·.1 == dst) |>.get!.2
  --   )
  --   |>.sum

  -- IO.println s!"Path: {path}"
  -- IO.println s!"Cost: {cost}"
