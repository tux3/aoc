import Lean.Data.HashMap
import Lean.Data.HashSet

abbrev Rules := List (Int × Int)
abbrev ReachabilityMap := Std.HashMap Int (Std.HashSet Int)

def Rules.reachability (rules: Rules) : ReachabilityMap :=
  rules.foldl (fun reach (a, b) =>
    reach
      |>.insertIfNew a Std.HashSet.empty
      |>.modify a (·.insert b)
  ) Std.HashMap.empty
