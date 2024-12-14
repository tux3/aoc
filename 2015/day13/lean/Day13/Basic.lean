import Std.Data.HashMap
import Batteries.Data.BinaryHeap

abbrev Idx := Nat
abbrev Cost := Int
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

def addFreeNode (g: Graph) (label: String): Graph :=
  let startIdx := g.labels.size
  let labels := g.labels.push label
  let edges := g.edges.push (Array.mkArray startIdx 0)
    |>.map (·.push 0)
  ⟨ labels, edges ⟩

-- When we put dst next to src, they both gain/lose happiness
def cost (graph: Graph) (src dst: Idx): Cost :=
  graph.edges[src]![dst]! + graph.edges[dst]![src]!

end Graph
