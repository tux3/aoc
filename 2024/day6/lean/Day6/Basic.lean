import Std.Data.HashSet

structure Map where
  grid: Array (Array Char)
  startPos: (Nat × Nat)
deriving Repr

inductive Direction
  | Up
  | Right
  | Down
  | Left
deriving Repr, DecidableEq, Hashable

def Direction.turn: Direction → Direction
| Up => Right
| Right => Down
| Down => Left
| Left => Up

namespace Map

def findStartPos (grid: Array (Array Char)) : (Nat × Nat) :=
  grid.mapIdx (·,·) |>.findSome? (fun (i, l) =>
    l.indexOf? '^' |>.map (·, i)
  ) |>.get!


def fromFile (path: System.FilePath) : IO Map := do
  let grid := (← IO.FS.readFile path).trimRight.splitOn "\n"
    |>.map (fun l => l.data.toArray)
    |>.toArray
  pure ⟨ grid, findStartPos grid ⟩

def nextPos (map: Map) (pos: Nat × Nat) (dir: Direction): Option (Nat × Nat) :=
  let (x, y) := pos
  match dir with
  | Direction.Up    => if y == 0 then none else (x, y-1)
  | Direction.Right => if x == map.grid.size-1 then none else (x+1, y)
  | Direction.Down  => if y == map.grid.size-1 then none else (x, y+1)
  | Direction.Left  => if x == 0 then none else (x-1, y)

def displayWalk (map: Map) (visited: Std.HashSet (Nat × Nat)) (obstructions: Std.HashSet (Nat × Nat)) : IO Unit := do
  for (y, l) in map.grid.mapIdx (·,·) do
    for (x, c) in l.mapIdx (·,·) do
      if obstructions.contains (x, y) then
        IO.print "O"
      else if visited.contains (x, y) then
        IO.print "x"
      else
        IO.print c
    IO.println ""

def canObstructAhead (map: Map) (obsPos : (Nat × Nat)) : Bool := Id.run do
  let mut pos := map.startPos
  let mut dir := Direction.Up
  let mut visited : Std.HashSet ((Nat × Nat) × Direction) := { }
  repeat
    if let some (x, y) := map.nextPos pos dir then
      if map.grid[y]![x]! == '#' || (x, y) == obsPos then
        dir := dir.turn
        let (hasLooped, visited') := visited.containsThenInsert (pos, dir)
        if hasLooped then return true
        visited := visited'
      else
        pos := (x, y)
    else break
  false

def walkResults (map: Map) : IO (Nat × Nat) := do
  let mut pos := map.startPos
  let mut dir := Direction.Up
  let mut visited : Std.HashSet (Nat × Nat) := { pos }
  let mut obstructions : Std.HashSet (Nat × Nat) := {}

  repeat
    if let some (x, y) := map.nextPos pos dir then
      if map.grid[y]![x]! == '#' then
        dir := dir.turn
      else
        pos := (x, y)
        visited := visited.insert pos
        if !obstructions.contains pos && map.canObstructAhead pos then
          obstructions := obstructions.insert pos
    else break

  map.displayWalk visited obstructions
  pure (visited.size, obstructions.size)

end Map
