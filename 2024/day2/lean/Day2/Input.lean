import Day2.Basic

def parseReports (input : String) : List Report :=
  input.splitOn "\n"
    |>.filter (!·.isEmpty)
    |>.map (·.splitOn.map String.toInt!)
    |>.map Report.mk

def parseReportsFile (path : System.FilePath) : IO (List Report) := do
  pure <| parseReports <| ← IO.FS.readFile path
