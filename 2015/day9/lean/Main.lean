import Day9

-- Held-Karp implementation, per Wikipedia's description =)
def bestPath (g: Graph) (comp: Nat→Nat→Bool): (Cost × Array String) := Id.run do
  -- Cache subset × target idx -> cost × best path
  let mut cache : Std.HashMap (List Idx × Idx) (Cost × Array Idx) := {}
  let (startIdx, g) := g.addStartNode
  let indexes := List.range startIdx

  for i in indexes do
    for dst in indexes.erase i do
      cache := cache.insert ([i, dst].mergeSort (·<·), i) (g.cost i dst, #[i, dst])

  for setSize in List.range (startIdx+1) |>.drop 3 do
    for set' in kCombinations setSize indexes do
      for dst in set' do
        let set := set'.erase dst
        cache := cache.insert (set', dst) $ bestByCost comp $ set.map (fun prev =>
          let (prevCost, prevPath) := cache.get! (set, prev)
          (prevCost + g.cost prev dst, prevPath.push dst))

  let (cost, path) := bestByCost comp (indexes.map (cache.get! ⟨indexes, ·⟩ ))
  pure (cost, path.map (g.labels[·]!))

def main : IO Unit := do
  let inputGraph := ← Graph.parseFile "../input"
  IO.println s!"Shortest path: {bestPath inputGraph (·<·)}"
  IO.println s!"Longest path: {bestPath inputGraph (·>·)}"
