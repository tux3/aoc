import Regex

structure Reindeer where
  name: String
  speed: Nat
  flyLen: Nat
  restLen: Nat

instance : Inhabited Regex.Captures where
  default := ⟨ "".toSubstring, #[] ⟩

instance : Inhabited Regex where
  default := ⟨ ⟨0, #[], false, by simp_arith⟩ ⟩

def parseInput (path: System.FilePath): IO (List Reindeer) := do
  let re := Regex.build r"^([a-zA-Z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+)" |>.toOption.get!
  pure $ (← IO.FS.readFile path).trimRight.splitOn "\n" |>.map (fun line =>
    let captures := (Regex.captures line.toSubstring re) |>.get!
    {
      name := captures.groups[0]!.get!.toString,
      speed := captures.groups[1]!.get!.toNat?.get!,
      flyLen := captures.groups[2]!.get!.toNat?.get!,
      restLen := captures.groups[3]!.get!.toNat?.get!
    }
  )

def flightDist (reindeer: Reindeer) (duration: Nat): Nat :=
  let cycleLen := reindeer.flyLen + reindeer.restLen
  let cycles := duration / cycleLen
  let rem := duration % cycleLen
  reindeer.speed * (cycles * reindeer.flyLen + (Nat.min reindeer.flyLen rem))

def bestFlightDist (reindeers: List Reindeer) (duration: Nat): (List String × Nat) :=
  let results := reindeers.map (fun reindeer =>
    (reindeer.name, flightDist reindeer duration)
  )
  let (_, maxDist) := results.toArray.max? (ord := ⟨(Ord.compare ·.2 ·.2)⟩) |>.get!
  (results.filter (fun (_, dist) => dist == maxDist) |>.map (·.1), maxDist)

def scorePoints (deers: List Reindeer) (duration: Nat): IO (String × Nat) := do
  let mut scores := Std.HashMap.empty
  for time in List.range duration do
    let (names, _dist) := bestFlightDist deers (time+1)
    for name in names do
      scores := scores.alter name (·.getD 0 |>.add 1)
  pure $ scores.toArray.max? (ord := ⟨(Ord.compare ·.2 ·.2)⟩) |>.get!

def main : IO Unit := do
  let duration := 2503
  let input := ← parseInput "../input"
  let (bestDeer, bestDist) := bestFlightDist input duration
  IO.println s!"{bestDeer[0]!} has run distance: {bestDist}"
  let (bestScoredDeer, bestScore) := ← scorePoints input duration
  IO.println s!"{bestScoredDeer} has scored {bestScore} points"
