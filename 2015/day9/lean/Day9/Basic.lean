import Std.Data.HashMap

def String.splitOnce (str: String) (pat: String): (String × String) :=
  let elems := str.splitOn pat
  (elems[0]!, elems[1]!)

abbrev Graph := Std.HashMap String (Array (String × Nat))

instance : ToString Graph where
  toString (graph: Graph): String :=
    let lines := graph.toArray.flatMap (fun (src, dests) =>
      dests.map (fun (dst, cost) => s!"{src} to {dst} = {cost}")
    ) |>.toList
    String.intercalate "\n" lines

namespace Graph

def parseFile (path: System.FilePath): IO Graph := do
  let mut graph := Std.HashMap.empty
  let lines := (← IO.FS.readFile path).trimRight.splitOn "\n"
  for line in lines do
    let (path, cost) := line.splitOnce " = "
    let (src, dst) := path.splitOnce " to "
    graph := graph.alter src (·.getD #[] |>.push (dst, cost.toNat!))
    graph := graph.alter dst (·.getD #[] |>.push (src, cost.toNat!))
  pure graph

def succs (graph: Graph) (node: String): Array String :=
  graph[node]!.map (·.1)

def preds (graph: Graph) (node: String): Array String :=
  graph.toArray.filterMap (fun (src, dests) =>
    if dests.map (fun (dst, _cost) => dst) |>.contains node then
      some src
    else none
  )

def removeNode (graph: Graph) (toRemove: String): Graph :=
  graph.filterMap (fun node dests =>
    if node == toRemove then
      none
    else
      dests.filter (fun (dst, _cost) => dst != toRemove)
  )

end Graph
