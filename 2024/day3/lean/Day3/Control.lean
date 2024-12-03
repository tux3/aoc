import Batteries.Data.String

structure Control where
  pos : String.Pos
  on : Bool
deriving Repr, BEq

instance : Inhabited Control where
  default := ⟨⟨0⟩, true⟩

def findControls (input : String) : List Control :=
  let rec orderControls : List Substring → List Substring -> List Control
    | [], [] => []
    | on :: ons, [] => ⟨ on.startPos, true ⟩ :: orderControls ons []
    | [], off :: offs => ⟨ off.startPos, false ⟩ :: orderControls [] offs
    | on :: ons, off :: offs =>
      if on.startPos < off.startPos then ⟨ on.startPos, true ⟩   :: orderControls ons (off :: offs)
      else                               ⟨ off.startPos, false ⟩ :: orderControls (on :: ons) offs
  orderControls (input.findAllSubstr "do()" |>.toList) (input.findAllSubstr "don't()" |>.toList)
