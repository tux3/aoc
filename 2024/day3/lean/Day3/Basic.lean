import Day3.Control
import Batteries.Data.String

def parseNumUntil(str: Substring) (stopChar: Char) : Option (Nat × Substring) :=
  let stopPos := str.posOf stopChar
  if stopPos == ⟨ str.bsize ⟩ then none
  else str.extract 0 stopPos
    |>.toNat?
    |>.map ((·, str.extract (str.next stopPos) str.stopPos))

def computeMul (input : String) (withControl : Bool) : Int := Id.run do
  -- We implicitly start with a do() at position 0
  let mut controls := ⟨⟨0⟩, true⟩ :: findControls input
  let mut count := 0

  for str in input.findAllSubstr "mul(" do
    -- Advance and check our do/don't controls
    while controls.tail != [] && str.startPos > controls[1]!.pos do
      controls := controls.tail
    if withControl && !controls.head!.on then continue

    -- Parse two numbers and multiply them (if control is on)
    let strCont: Substring := ⟨ input, str.drop 4 |>.startPos, input.endPos ⟩
    if let some (n1, afterN1) := parseNumUntil strCont ',' then
      if let some (n2, _afterN2) := parseNumUntil afterN1 ')' then
        count := count + n1 * n2
  pure count
