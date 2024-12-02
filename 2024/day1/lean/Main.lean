import Day1

def main : IO Unit := do
  let (left, right) := (← readInputFile "../input-test")
    |> Prod.map List.mergeSort List.mergeSort

  IO.println s!"Distance (test): {computeDistance left right}"
  IO.println s!"Similarity (test): {computeSimilarity left right}"

  let (left, right) := (← readInputFile "../input")
    |> Prod.map List.mergeSort List.mergeSort

  IO.println s!"Distance (input): {computeDistance left right}"
  IO.println s!"Similarity (input): {computeSimilarity left right}"
