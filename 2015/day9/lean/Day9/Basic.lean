import Std.Data.HashMap
import Batteries.Data.BinaryHeap

abbrev Idx := Nat
abbrev Cost := Nat
abbrev Edges := Array (Array (Cost)) -- src -> dst -> cost

structure Graph where
  labels: Array String
  edges: Edges

instance : ToString Graph where
  toString (g: Graph): String :=
    let lines := g.edges.mapIdx (·,·) |>.flatMap (fun (src, dests) =>
      dests.mapIdx (fun (dst cost) => s!"{g.labels[src]!} to {g.labels[dst]!} = {cost}")
    ) |>.toList
    String.intercalate "\n" lines

namespace Graph

def addStartNode (g: Graph): (Idx × Graph) :=
  let startIdx := g.labels.size
  let labels := g.labels.push "Start"
  let edges := g.edges.push (Array.mkArray startIdx 0)
    |>.map (·.push 0)
  (startIdx, ⟨ labels, edges ⟩)

def removeNode (graph: Graph) (idx: Idx): Graph :=
  { graph with edges := (graph.edges.eraseIdx idx).map (·.eraseIdx idx) }

end Graph
