import Mathlib.Data.Matrix.Basic

inductive Op
  | Toggle
  | On
  | Off
deriving Repr, Inhabited, BEq

abbrev GridSize := 1000
abbrev Grid := Matrix (Fin GridSize) (Fin GridSize) Nat
abbrev Box := (Nat × Nat × Nat × Nat) -- x1, y1, x2, y2

def range := List.range GridSize

def numLights (mat: Grid) : Nat :=
  List.product range range |>.foldl (fun acc (x, y) => acc + (mat x y)) 0

def execOp1 (op: Op) (prev: Nat): Nat := match op with
  | Op.Toggle => 1 - prev
  | Op.On => 1
  | Op.Off => 0

def execOp2 (op: Op) (prev: Nat): Nat := match op with
  | Op.Toggle => prev + 2
  | Op.On => prev + 1
  | Op.Off => prev - 1

def execList (instr: List (Op × Box)) (execOp: Op → Nat → Nat): Nat :=
  instr.foldl (fun grid (op, (x1, y1, x2, y2)) =>
    Matrix.of (fun x y =>
      if x >= x1 && x <= x2 && y >= y1 && y <= y2
      then execOp op (grid x y)
      else grid x y
    )
  ) (Matrix.of 0)
  |> numLights
