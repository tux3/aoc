import Day6.Basic

def parseOp (str: String) : (Op × Substring) :=
  if str.startsWith "toggle" then
    (Op.Toggle, ⟨ str, ⟨"toggle ".length⟩, str.endPos ⟩)
  else if str.startsWith "turn on" then
    (Op.On, ⟨ str, ⟨"turn on ".length⟩, str.endPos ⟩)
  else if str.startsWith "turn off" then
    (Op.Off, ⟨ str, ⟨"turn off ".length⟩, str.endPos ⟩)
  else panic! "Invalid input line"

def parseNatPair(str: Substring): (Nat × Nat) :=
  let elems := str.splitOn "," |>.map (·.toString.toNat!)
  (elems[0]!, elems[1]!)

def parseLine (str: String) : (Op × Box) :=
  let (op, str) := parseOp str
  let elems := str.splitOn " through "
  let (x1, y1) := parseNatPair elems[0]!
  let (x2, y2) := parseNatPair elems[1]!
  (op, (x1, y1, x2, y2))

#guard parseLine "toggle 461,550 through 564,900" == (Op.Toggle, (461, 550, 564, 900))
#guard parseLine "turn off 812,389 through 865,874" == (Op.Off, (812, 389, 865, 874))
#guard parseLine "turn on 599,989 through 806,993" == (Op.On, (599, 989, 806, 993))
