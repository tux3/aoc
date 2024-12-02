import Std

private def List.frequencyCounts [BEq α] [Hashable α] (list: List α) : Std.HashMap α Int := Id.run do
  let mut freq : Std.HashMap α Int := Std.HashMap.empty
  for e in list do
    freq := match freq[e]? with
      | none => freq.insert e 1
      | some n => freq.insert e (n+1)
  pure freq

def computeDistance (left: List Int) (right: List Int) : Int :=
  List.zip left right
    |>.map (fun (l, r) => (r - l).natAbs)
    |>.sum

def computeSimilarity (left: List Int) (right: List Int) : Int :=
  let freq: Std.HashMap Int Int := right.frequencyCounts
  left.map (fun l => match freq[l]? with
      | none => 0
      | some n => l * n
    )
    |> List.sum
