import Day10

-- Not tail rec, unfortunately! :')
def lookAndSayRec (l: List Nat): List Nat :=
  match l with
  | [] => []
  | [x] => [1, x]
  | x1 :: x2 :: xs =>
    let next := lookAndSayRec (x2 :: xs)
    if x1 == x2 then
      (next.head! + 1) :: next.tail!
    else
      1 :: x1 :: next

def lookAndSay (l: List Nat): List Nat := Id.run do
  let mut last := l[0]!
  let mut count := 1
  let mut result := #[]
  for c in l.drop 1 do
    if c == last then
      count := count + 1
    else
      result := result.append #[count, last]
      last := c
      count := 1
  result := result.append #[count, last]
  pure result.toList

def main : IO Unit := do
  let input := (← IO.FS.readFile "../input").trimRight.toList.map (·.toNat - 0x30)
  IO.println s!"Input: {input}"
  let mut cur := input
  for i in List.range 50 do
    cur := lookAndSay cur
    IO.println s!"Step {i+1}: {cur.length} digits"
