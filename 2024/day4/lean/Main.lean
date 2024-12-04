import Day4

def main : IO Unit := do
  IO.println s!"Count (input-test): {wordsearch (← Grid.fromFile "../input-test")}"
  IO.println s!"Count (input): {wordsearch (← Grid.fromFile "../input")}"

  IO.println s!"X-Count (input-test): {crosssearch (← Grid.fromFile "../input-test")}"
  IO.println s!"X-Count (input): {crosssearch (← Grid.fromFile "../input")}"
