import Mathlib.Data.Matrix.Basic

structure Ingredient where
  name: String
  coeffs: Array Int
  calories: Nat
deriving Repr, Inhabited

def parseInput (str: String): Array Ingredient :=
  str.trimRight.splitOn "\n" |>.toArray.map (fun l =>
    let elems := l.splitOn ": "
    let name := elems[0]!
    let elems := elems[1]!.splitOn ", "
    let elems := elems.map (·.splitOn[1]!.toInt!)
    let calories := elems.getLast!.toNat
    let coeffs := elems.dropLast.toArray
    ⟨ name, coeffs, calories ⟩
  )

def score (amounts: List Nat) (coeffs: Array (Array Int)): Nat :=
  let amountsVec: (Fin amounts.length) → Int := (amounts[·]!)
  let coeffsMat := Matrix.of (fun i j => (coeffs[i]!)[j]!)
  let res: Nat → Int := Matrix.vecMul amountsVec coeffsMat
  (List.range coeffs[0]!.size).foldl (fun acc i => acc * (res i).toNat) 1

-- This is absolutely terrible, but lean matrices are so hard to work with,
-- I gave up on clever linear algebra and did a brute force :')
partial def genAllAmounts (maxSum numVars: Nat): List (List Nat) := Id.run do
  if numVars == 1 then
    return [[maxSum]]
  let mut allResults := []
  for a in List.range (maxSum+1) do
    for r in genAllAmounts (maxSum-a) (numVars -1) do
      allResults := (a :: r) :: allResults
  allResults

def main : IO Unit := do
  let ingredients := parseInput (← IO.FS.readFile "../input")
  let coeffs := ingredients.map (·.coeffs)

  let allAmounts := genAllAmounts 100 ingredients.size
  let best := allAmounts.foldl (fun prev amounts =>
    let cur := score amounts coeffs
    if cur >= prev then cur else prev
  ) 0
  IO.println s!"best score: {best}"

  let bestWithCal := allAmounts.foldl (fun prev amounts =>
    let cur := score amounts coeffs
    let calories := amounts.mapIdx (fun idx n => ingredients[idx]!.calories * n) |>.sum
    if calories == 500 && cur >= prev then cur else prev
  ) 0
  IO.println s!"best score with calories: {bestWithCal}"
