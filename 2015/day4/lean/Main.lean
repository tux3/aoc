-- This is not a place of honor
-- No one's implemented MD5 in Lean, and I don't feel like going through all of that just for one 2015 AoC prompt :')
-- Let's see how fast Linux's process creation is...
def md5sum (str: String): IO String := do
  let args : IO.Process.SpawnArgs := { cmd := "md5sum", stdin := .piped }
  let child ← IO.Process.spawn { args with stdout := .piped, stderr := .piped }
  child.stdin.putStr str
  let (_, child) := ← child.takeStdin

  let stdout ← IO.asTask child.stdout.readToEnd Task.Priority.dedicated
  let _exitCode ← child.wait
  let stdout ← IO.ofExcept stdout.get
  pure stdout

def main (args : List String) : IO Unit := do
  -- Oh god it wants 6 zeroes for part 2
  -- Let's double down on the madness and parallelize the search... @_@
  -- (It does find it, but only after starting at 9900000 and having tried all 99 previous values !)
  let start := args[0]?.map (·.toNat!) |>.getD 0

  let input := "yzbqklnj"
  let rec brute (max i : Nat): IO Nat := do
    let hash := (← md5sum s!"{input}{i}")
    if i % 10000 == 0 then
      IO.println s!"{i} -> {hash.trimRight}"
    match max with
    | 0 => panic! "Max iterations reached..."
    | max+1 =>
      if hash.startsWith "000000" then pure i
      else brute max (i+1) -- Somehow this is not tail-recursive..! Bye bye stack!

  IO.println s!"Result: {input}{← brute 100000 start}"
