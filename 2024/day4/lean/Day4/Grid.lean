structure Grid where
  lines : Array String
deriving Repr

namespace Grid

def fromFile (path: System.FilePath): IO Grid := do
  let input := ← IO.FS.readFile path
  pure ⟨ input.trimRight.splitOn "\n" |>.toArray ⟩

def columns (g: Grid): Array String :=
  g.lines.mapIdx (fun i _ => ⟨ g.lines.map (·.get! ⟨i⟩) |>.toList ⟩)

def diagonals (g: Grid): Array String :=
  let n := g.lines.size
  (List.range (n-3)
    |>.flatMap (fun i => [
      ⟨ (List.range (n-i)).map (fun j => g.lines[i+j]!.get! ⟨j⟩ ) ⟩,
      ⟨ (List.range (n-i)).map (fun j => g.lines[n-1 - (i+j)]!.get! ⟨j⟩ ) ⟩,
    ])
  ).append (List.range (n-3)
    |>.tail! -- Don't process center diagonal twice
    |>.flatMap (fun i => [
      ⟨ (List.range (n-i)).map (fun j => g.lines[j]!.get! ⟨i+j⟩ ) ⟩,
      ⟨ (List.range (n-i)).map (fun j => g.lines[n-1 - j]!.get! ⟨i+j⟩ ) ⟩,
    ])
  )
  |>.toArray

def crosses (g: Grid): Array (String × String) :=
  let n := g.lines.size
  (List.range (n-2) |>.flatMap (fun i =>
    (List.range (n-2) |>.map (fun j => (
      ⟨ (List.range (3)).map (fun k => g.lines[i+k]!.get! ⟨j+k⟩ ) ⟩,
      ⟨ (List.range (3)).map (fun k => g.lines[i+2 - k]!.get! ⟨j+k⟩ ) ⟩
    )))
  ))
  |>.toArray

end Grid
