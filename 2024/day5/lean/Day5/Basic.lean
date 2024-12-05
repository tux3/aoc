import Day5.Rules
import Day5.Update

def parseFile (path : System.FilePath) : IO (Rules × List Update) := do
  let input := (← IO.FS.readFile path).trimRight.splitOn "\n\n"

  let rules := input[0]!.splitOn "\n" |>.map (fun l =>
    let elems := l.splitOn "|"
    (elems[0]!.toInt!, elems[1]!.toInt!)
  )

  let updates := input[1]!.splitOn "\n" |>.map (fun l =>
    l.splitOn "," |>.map (·.toInt!)
  )
  pure (rules, updates)

def sumMiddlePagesForFile (path : System.FilePath) : IO (Int × Int) := do
  let (rules, updates) := ← parseFile path
  let reach := rules.reachability
  let validSum := updates.filter (·.isValid reach) |>.map (·.middlePage) |>.sum
  let invalidSum := updates.filter (!·.isValid reach) |>.map (·.sorted reach |>.middlePage) |>.sum
  pure (validSum, invalidSum)
