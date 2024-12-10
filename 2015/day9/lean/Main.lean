import Day9

-- Held-Karp implementation, per Wikipedia's description =)
def bestPath (g: Graph) (comp: Nat→Nat→Bool): IO (Cost × Array String) := do
  -- Cache subset × target idx -> cost × best path
  let mut cache : Std.HashMap (List Idx × Idx) (Cost × Array Idx) := {}
  let (startIdx, g) := g.addStartNode
  let indexes := List.range startIdx

  for i in indexes do
    for pair in kCombinations 2 (indexes.erase i) do
      let (cost, path) := shortestPathBruteforce startIdx i pair g comp
      cache := cache.insert (path.mergeSort (·<·), i) (cost, path.toArray)

  for setSize in List.range (startIdx+1) |>.drop 4 do
    for set' in kCombinations setSize indexes do
      for dst in set' do
        let set := set'.erase dst
        cache := cache.insert (set', dst) $ bestByCost comp $ set.map (fun prev =>
          let (prevCost, prevPath) := cache.get! (set, prev)
          let cost := prevCost + g.edges[prev]![dst]!
          (cost, prevPath.push dst)
        )

  let (cost, path) := bestByCost comp (indexes.map (cache.get! ⟨indexes, ·⟩ ))
  pure (cost, path.map (g.labels[·]!))

def main : IO Unit := do
  let inputGraph := ← Graph.parseFile "../input"
  IO.println s!"Shortest path: {← bestPath inputGraph (·<·)}"
  IO.println s!"Longest path: {← bestPath inputGraph (·>·)}"
