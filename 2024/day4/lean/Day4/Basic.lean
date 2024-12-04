import Day4.Grid
import Batteries.Data.String

def wordsearch (grid: Grid) : Nat :=
  #[grid.lines, grid.columns, grid.diagonals].flatten.map (fun str =>
    (str.findAllSubstr "XMAS").size + (str.findAllSubstr "SAMX").size
  ) |>.toList.sum

def crosssearch (grid: Grid) : Nat :=
  grid.crosses.map (fun (a, b) =>
    (a == "MAS" || a == "SAM") && (b == "MAS" || b == "SAM") |>.toNat
  ) |>.toList.sum
