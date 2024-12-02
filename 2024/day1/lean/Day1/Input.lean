def readInputFile (path: System.FilePath) : IO (List Int × List Int) := do
  (← IO.FS.readFile path)
    |>.splitOn "\n"
    |>.filter (!·.isEmpty)
    |>.map String.splitOn
    |>.map (fun words => (words[0]!.toInt!, words[1]!.toInt!))
    |>.unzip
    |> pure
