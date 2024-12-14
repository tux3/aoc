-- Due to the strcture of lists, this increments at the start, so we reverse the lists
def incr (l: List Char): List Char :=
  match l with
  | [] => []
  | c :: cs => if c == 'z' then
     'a' :: incr cs
    else
      (Char.ofNat (c.toNat+1)) :: cs

-- Really looking for a decreasing straight, because of reversed lists
def hasIncreasingStraight: (List Char) → Bool
| c1 :: c2 :: c3 :: cs => (c1.toNat == 1+c2.toNat && c2.toNat == 1+c3.toNat) || hasIncreasingStraight (c2 :: c3 :: cs)
| _ => false

def numPairs: (List Char) → Nat
| c1 :: c2 :: cs => if c1 == c2 then 1 + numPairs cs else numPairs (c2 :: cs)
| _ => 0

def isValid (l: List Char): Bool :=
  hasIncreasingStraight l
    && numPairs l >= 2
    && ! l.any (fun c => ['i', 'o', 'l'].contains c)

#guard !isValid "hijklmmn".toList.reverse
#guard isValid "abcdffaa".toList.reverse
#guard !isValid "hijklmmn".toList.reverse
#guard isValid "ghjaabcc".toList.reverse

partial def nextValid (l: List Char): List Char :=
  if isValid l then l else nextValid (incr l)

#guard nextValid "abcdefgh".toList.reverse == "abcdffaa".toList.reverse

def main : IO Unit := do
  let input := (← IO.FS.readFile "../input").trimRight.toList
  IO.println s!"Input: {input.asString}"
  let pass1 := (nextValid input.reverse).reverse
  IO.println s!"Next valid pass (part 1): {pass1.asString}"
  let pass2 := (nextValid (incr pass1.reverse)).reverse
  IO.println s!"Next valid pass (part 2): {pass2.asString}"
