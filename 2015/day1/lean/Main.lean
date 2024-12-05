def lispVal: Char → Int
  | '(' => 1
  | ')' => -1
  | _ => 0

def evalLisp (str: String) : Int :=
  str.toList.map lispVal |>.sum

def evalLispUntilNeg (str: String) : Int :=
  str.toList.mapIdx (fun i c => (i,c) )
    |>.foldl (fun (accIdx, accVal) (i, c) =>
      if accVal < 0 then (accIdx, accVal)
      else (i+1, accVal + lispVal c)
    ) (0, 0)
    |>.1

def main : IO Unit := do
  let input := ← IO.FS.readFile "../input"
  IO.println s!"Value: {evalLisp input}"
  IO.println s!"First neg pos: {evalLispUntilNeg input}"
