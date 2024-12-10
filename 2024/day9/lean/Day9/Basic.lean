
-- (fileIdx, numFileBlocks, numFreeBlocks)
abbrev Slot := (Nat × Nat × Nat)

def slotToBlocks: Slot → List (Option Nat)
| (fileIdx, nFileBlocks, nFreeBlocks) =>
  Array.ofFn (n := nFileBlocks) (λ _ => some fileIdx)
  ++ Array.ofFn (n := nFreeBlocks) (λ _ => none)
  |>.toList

def inputMapToBlocks (inputMap: List Nat): List (Option Nat) :=
  let rec inputMapToBlocks (inputMap: List Nat) (idx: Nat): List (Option Nat) :=
    match inputMap with
    | nFile :: nFree :: tail =>
      slotToBlocks (idx, nFile, nFree) ++ inputMapToBlocks tail (idx+1)
    | [nFile] => (List.range nFile).map (fun _ => idx)
    | [] => []
  inputMapToBlocks inputMap 0

-- Part 1: Compact blocks into free space one by one
def packBlocks (inputBlocks: List (Option Nat)): List (Option Nat) :=
  let nFiles := inputBlocks.map (·.isSome.toNat) |>.sum
  let rec popNextBlock: List (Option Nat) → (Nat × (List (Option Nat)))
    | some n :: tail => (n, tail)
    | none :: tail => popNextBlock tail
    | [] => panic!("Tried to pop block from list, but reached the end")

  let rec doPackBlocks (front back: List (Option Nat)) (remaining: Nat) :=
    match remaining with
    | 0 => []
    | n + 1 => match front with
      | [] => []
      | some block :: tail => block :: doPackBlocks tail back n
      | none :: tail => let (backBlock, backTail) := popNextBlock back
        backBlock :: doPackBlocks tail backTail n
  doPackBlocks inputBlocks inputBlocks.reverse nFiles

-- Part 2: Compact entire files into free space
def packFileBlocks (inputMap : List Nat): List (Option Nat) := Id.run do
  let rec slotsWithIdx (inputMap : List Nat ) (idx : Nat): List Slot :=
    match inputMap with
    | nFile :: nFree :: tail => (idx, nFile, nFree) :: slotsWithIdx tail (idx+1)
    | [nFile] => [(idx, nFile, 0)]
    | [] => []
  let mut slots := slotsWithIdx inputMap 0 |>.toArray

  -- First slot with at least fileSize empty space
  let findFreeSpace (files: Array Slot) (fileSize stopIdx: Nat): Option Nat := Id.run do
    for (i, fileIdx, _nFile, nFree) in files.mapIdx (·,·) do
      if fileIdx == stopIdx then
        return none
      if nFree >= fileSize then
        return some i
    none

  for i in List.range slots.size |>.reverse do
    let mut pos := slots.findIdx? (·.1 == i) |>.get!
    let mut (fileIdx, nFile, nFree) := slots[pos]!

    if let some freePos := findFreeSpace slots nFile fileIdx then
      let (otherIdx, otherNFile, otherNFree) := slots[freePos]!
      slots := slots.set! pos (fileIdx, 0, nFree + nFile)
      slots := slots.set! freePos (otherIdx, otherNFile, 0)
      slots := slots.insertAt! (freePos+1) (fileIdx, nFile, otherNFree - nFile)

  return slots.toList.flatMap slotToBlocks
