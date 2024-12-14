import Day13.Basic
import Day13.Utils
import Regex

instance : Inhabited Regex.Captures where
  default := ⟨ "".toSubstring, #[] ⟩

instance : Inhabited Regex where
  default := ⟨ ⟨0, #[], false, by simp_arith⟩ ⟩

def Graph.parseFile (path: System.FilePath): IO Graph := do
  let mut edgesMap := Std.HashMap.empty
  let lines := (← IO.FS.readFile path).trimRight.splitOn "\n"
  let mut labels: Array String := #[]
  let mut labelsIdx := Std.HashMap.empty
  let re := Regex.build r"^([a-zA-Z]+) would (gain|lose) (\d+) happiness units by sitting next to ([a-zA-Z]+).$" |>.toOption.get!
  for line in lines do
    let captures := (Regex.captures line re) |>.get!
    let (src, dst) := (captures.groups[0]!.get!.toString, captures.groups[3]!.get!.toString)
    let isGain := captures.groups[1]!.get! == "gain"
    let costNat := captures.groups[2]!.get!.toNat?.get!
    let cost: Cost := if isGain then costNat else -costNat

    if !labelsIdx.contains src then
        labelsIdx := labelsIdx.insert src labels.size
        labels := labels.push src
    if !labelsIdx.contains dst then
      labelsIdx := labelsIdx.insert dst labels.size
      labels := labels.push dst
    let srcIdx := labelsIdx[src]!
    let dstIdx := labelsIdx[dst]!
    edgesMap := edgesMap.alter srcIdx (·.getD #[] |>.push (dstIdx, cost))

  let edges := edgesMap.toArray.heapSort (·.1 < ·.1)
    |>.map (fun (src, dsts) => dsts.push (src, 0) |>.heapSort (·.1 < ·.1) |>.map (·.2))

  for dsts in edges do
    if dsts.size != edges.size then panic!("Unexpected missing edges")

  pure ⟨ labels, edges ⟩
