import Lean.Data.HashSet

def move (s: Std.HashSet (Int×Int)) (c: Char) (x y: Int): Std.HashSet (Int×Int) × Int × Int :=
  let (x', y') := match c with
    | '<' => (x-1, y)
    | '^' => (x, y+1)
    | '>' => (x+1, y)
    | 'v' => (x, y-1)
    | _ => (x, y)
  (s.insert (x', y'), x', y')

def santaCount (input: String) : Int :=
  input.foldl (fun (s, x, y) c =>
    move s c x y
  ) ({(0, 0)}, 0, 0)
    |>.1
    |>.size

def santaAndBotCount (input: String) : Int :=
  input.foldl (fun (s, x1, y1, x2, y2) c =>
    let (s', x', y') := move s c x1 y1
    (s', x2, y2, x', y')
  ) ({(0, 0)}, 0, 0, 0, 0)
    |>.1
    |>.size

def main : IO Unit := do
  let input := (← IO.FS.readFile "../input")
  IO.println s!"Santa count: {santaCount input}"
  IO.println s!"Santa and bot count: {santaAndBotCount input}"

-- Simple tests

#guard (santaCount "^") == 2
#guard (santaCount "^>v<") == 4
#guard (santaCount "^v^v^v^v^v") == 2

#guard (santaAndBotCount "^v") == 3
#guard (santaAndBotCount "^>v<") == 3
#guard (santaAndBotCount "^v^v^v^v^v") == 11
