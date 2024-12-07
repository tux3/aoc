import Batteries.Data.String

def isNice (s: String): Bool :=
  (s.toList.map ("aeiou".toList.contains · |>.toNat) |>.sum) >= 3
  && (s.toList.zip (s.toList.tail!)).any (fun (a, b) => a == b)
  && !((s.toList.zip (s.toList.tail!)).any (fun (a, b) =>
    [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains (a, b)
  ))

def isNice2 (s: String): Bool :=
  let sl := s.toList
  let pairs := (sl.zip (sl.tail!)).map (fun (a, b) => s!"{a}{b}")
  let hasRepeatPair := pairs.any (fun p =>
    s.findSubstr? p >>= ({ s.toSubstring with startPos := ·.stopPos }.findSubstr? p) |>.isSome
  )
  let hasAba := (sl.zip (sl.tail!.tail!)).any (fun (a, b) => a == b)
  hasRepeatPair && hasAba

def main : IO Unit := do
  let input := (← IO.FS.readFile "../input").trimRight.splitOn "\n"
  IO.println s!"Nice strings part 1: {input.map (isNice · |>.toNat) |>.sum}"
  IO.println s!"Nice strings part 2: {input.map (isNice2 · |>.toNat) |>.sum}"


#guard isNice2 "qjhvhtzxzqqjkmpb"
#guard isNice2 "xxyxx"

#guard !isNice2 "uurcxstgmygtbstg"
#guard !isNice2 "ieodomkazucvgmuy"
#guard !isNice2 "aaa"
