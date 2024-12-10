import Day9.Basic
import Day9.Utils

def kCombinations (k: Nat) (l: List Nat): List (List Nat) :=
  match k with
  | 0 => panic!("kCombinations input list smaller than k elements")
  | 1 => l.map ([·])
  | n+1 => l.flatMap (fun x =>
      let xs := l.erase x
      (kCombinations n xs)
      |>.filter (x < ·[0]!)
      |>.map (x :: ·)
    )

#guard (kCombinations 3 [4, 3, 1, 2]) == [[1, 3, 4], [1, 2, 4], [1, 2, 3], [2, 3, 4]]

-- Returns cost × path
partial def shortestPathBruteforce (src dst: Idx) (through: List Idx)
                                   (g: Graph) (comp: Nat→Nat→Bool): (Nat × List Idx) :=
  if through.isEmpty then
    (g.edges[src]![dst]!, [dst])
  else
    let paths := through.map (fun i =>
      let rest := through.erase i
      let startCost := g.edges[src]![i]!
      let (nextCost, nextPath) := shortestPathBruteforce i dst rest g comp
      (startCost + nextCost, i :: nextPath)
    )
    bestByCost comp paths
