import Day9.Basic

def String.splitOnce (str: String) (pat: String): (String × String) :=
  let elems := str.splitOn pat
  (elems[0]!, elems[1]!)

def bestByCost [Inhabited α] (comp: (Cost→Cost→Bool)) (paths: List (Cost × α)): (Cost × α) :=
  paths.foldl (fun (bestCost, bestPath) (cost, path) =>
    if comp bestCost cost then (bestCost, bestPath)
    else (cost, path)) paths[0]!

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
