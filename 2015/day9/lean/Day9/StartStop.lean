import Day9.Basic

namespace Graph

-- If a node cannot be reached, we can't start anywhere else
-- So we remove it from the graph and start from one of its successors
structure StartingPathResult where
  forcedStartPath: Array String
  prunedGraph: Graph
  allowedStartNodes: Array String

def allowedStartingPath (startGraph: Graph): StartingPathResult := Id.run do
  let mut forcedStartPath: Array String := #[]
  let mut graph := startGraph
  let mut allowedStartNodes := graph.keys.toArray

  repeat
    if let some forcedStart := graph.keys.find? (fun node => graph.preds node |>.isEmpty) then
      forcedStartPath := forcedStartPath.push forcedStart
      allowedStartNodes := graph.succs forcedStart
      graph := graph.removeNode forcedStart
    else break

  ⟨ forcedStartPath, graph, allowedStartNodes ⟩

end Graph
