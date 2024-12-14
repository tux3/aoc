import Lean.Data.Json

partial def sumJson: Lean.Json → Int
| .num n => n.toString.toInt!
| .arr a => a.map sumJson |>.toList.sum
| .obj o => o.fold (fun acc _k v => acc + sumJson v) 0
| _ => 0

partial def sumJsonWithoutRed: Lean.Json → Int
| .num n => n.toString.toInt!
| .arr a => a.map sumJsonWithoutRed |>.toList.sum
| .obj o =>
  if o.any (fun _k v => v == .str "red") then 0
  else o.fold (fun acc _k v => acc + sumJsonWithoutRed v) 0
| _ => 0

def main : IO Unit := do
  let input := Lean.Json.parse (← IO.FS.readFile "../input") |>.toOption.get!
  IO.println s!"Sum: {sumJson input}"
  IO.println s!"Sum without red: {sumJsonWithoutRed input}"
