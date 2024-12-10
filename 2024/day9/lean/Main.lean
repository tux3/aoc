import Day9

def checksum (blocks: List (Option Nat)): Nat :=
  blocks.mapIdx (fun i n => i * n.getD 0) |>.sum

def checksumsForFile (path: System.FilePath): IO (Nat × Nat) := do
  let inputMap := (← IO.FS.readFile path).trimRight.toList.map (·.toNat - 0x30)
  let packedBlocks := packBlocks (inputMapToBlocks inputMap)
  let packedFilesBlocks := packFileBlocks inputMap
  pure (checksum packedBlocks, checksum packedFilesBlocks)

def main : IO Unit := do
  IO.println s!"Test checksums: {← checksumsForFile "../input-test"}"
  IO.println s!"Input checksums: {← checksumsForFile "../input"}"
