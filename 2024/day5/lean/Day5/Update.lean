import Day5.Rules

abbrev Update := List Int

namespace Update

def isValid (update: Update) (reach: ReachabilityMap) : Bool :=
  update.zip (update.tail!) |>.all (fun (a, b) =>
    reach.contains a && reach[a]!.contains b
  )

def middlePage (update: Update) : Int :=
  -- We assume updates have an odd number of elements
  let rec findMiddle: List Int → List Int → Int
    | _a :: [], b :: _bs => b
    | _a1 :: _a2 :: as, _b :: bs => findMiddle as bs
    | _, _ => panic! "Unreachable pattern in Update.middlePage"
  findMiddle update update

def sorted (update: Update) (reach: ReachabilityMap) : Update :=
  update.mergeSort (fun a b =>
    if reach[a]?.map (·.contains b) == some true then true
    else if reach[b]?.map (·.contains a) == some true then false
    else true
  )

end Update
