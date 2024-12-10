import Day9.Basic
import Day9.Utils

def Graph.parseFile (path: System.FilePath): IO Graph := do
  let mut edgesMap := Std.HashMap.empty
  let lines := (← IO.FS.readFile path).trimRight.splitOn "\n"
  let mut labels: Array String := #[]
  let mut labelsIdx := Std.HashMap.empty
  for line in lines do
    let (path, cost) := line.splitOnce " = "
    let (src, dst) := path.splitOnce " to "
      if !labelsIdx.contains src then
        labelsIdx := labelsIdx.insert src labels.size
        labels := labels.push src
    if !labelsIdx.contains dst then
      labelsIdx := labelsIdx.insert dst labels.size
      labels := labels.push dst
    let srcIdx := labelsIdx[src]!
    let dstIdx := labelsIdx[dst]!
    edgesMap := edgesMap.alter srcIdx (·.getD #[] |>.push (dstIdx, cost.toNat!))
    edgesMap := edgesMap.alter dstIdx (·.getD #[] |>.push (srcIdx, cost.toNat!))

  let edges := edgesMap.toArray.heapSort (·.1 < ·.1)
    |>.map (fun (src, dsts) => dsts.push (src, 0) |>.heapSort (·.1 < ·.1) |>.map (·.2))

  for dsts in edges do
    if dsts.size != edges.size then panic!("Unexpected missing edges")

  pure ⟨ labels, edges ⟩
