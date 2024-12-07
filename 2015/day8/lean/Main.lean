def decodeDiff (str: String): Nat :=
  let (diff, _) := str.toList.foldl (fun (acc, isEscaped) c =>
    if isEscaped then
      match c with
      | '\\' | '"' => (acc + 1, false)
      | 'x' => (acc + 3, false)
      | _ => panic! s!"Unexpected escape \\{c}"
    else
      (acc, c == '\\')
  ) (2, false)
  diff

def encodeDiff (str: String): Nat :=
  str.toList.foldl (fun acc c =>
    match c with
    | '\\' | '"' => acc + 1
    | _ => acc
  ) 2

def unescapedDiffSum (path: System.FilePath): IO (Nat × Nat) := do
  let input := (← IO.FS.readFile path).trimRight.splitOn "\n"
  let diff1 := input.map decodeDiff |>.sum
  let diff2 := input.map encodeDiff |>.sum
  pure (diff1, diff2)

def main : IO Unit := do
  IO.println s!"Result (test): {← unescapedDiffSum "../input-test"}"
  IO.println s!"Result (input): {← unescapedDiffSum "../input"}"
