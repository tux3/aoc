import Day9.Basic

def String.splitOnce (str: String) (pat: String): (String × String) :=
  let elems := str.splitOn pat
  (elems[0]!, elems[1]!)

def bestByCost [Inhabited α] (comp: (Cost→Cost→Bool)) (paths: List (Cost × α)): (Cost × α) :=
  paths.foldl (fun (bestCost, bestPath) (cost, path) =>
    if comp bestCost cost then (bestCost, bestPath)
    else (cost, path)) paths[0]!
