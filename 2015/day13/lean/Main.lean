import Day13

-- Held-Karp implementation from day 9, but adapted for a cycle!
def bestPath (g: Graph) (comp: Cost→Cost→Bool): IO (Cost × Array Idx) := do
  -- Cache subset × target idx -> cost × best path
  let mut cache : Std.HashMap (List Idx × Idx) (Cost × Array Idx) := {}
  let indexes := List.range g.labels.size

  for i in indexes do
    for dst in indexes.erase i do
      cache := cache.insert ([i, dst].mergeSort (·<·), dst) (g.cost i dst + g.cost dst i, #[i, dst])

  for setSize in List.range (g.labels.size+1) |>.drop 3 do
    for set' in kCombinations setSize indexes do
      for dst in set' do
        let set := set'.erase dst
        cache := cache.insert (set', dst) $ bestByCost comp $ set.map (fun prev =>
          let (prevCost, prevPath) := cache.get! (set, prev)
          (prevCost - g.cost prev prevPath[0]! + g.cost prev dst + g.cost dst prevPath[0]!, prevPath.push dst))

  return bestByCost comp (indexes.map (cache.get! ⟨indexes, ·⟩))

def main : IO Unit := do
  let graph := ← Graph.parseFile "../input"
  let (cost, path) := ← bestPath graph (·>·)
  IO.println s!"Graph:\n{graph}"
  IO.println s!"Best arrangement: {cost} {path.map (graph.labels[·]!)}"

  let graphWithMe := graph.addFreeNode "Me"
  let (cost, path) := ← bestPath graphWithMe (·>·)
  IO.println s!"Best arrangement with me: {cost} {path.map (graphWithMe.labels[·]!)}"
