def main : IO Unit := do
  let dims := (← IO.FS.readFile "../input")
    |>.trimRight
    |>.splitOn "\n"
    |>.map (fun l =>
      let elems := l.splitOn "x"
      (elems[0]!.toInt!, elems[1]!.toInt!, elems[2]!.toInt!)
    )
  let paper := dims.map (fun (w, l, h) =>
      let (a, b, c) := (w*l, l*h, w*h)
      2 * (a + b + c) + [a, b, c].min?.get!
    ) |>.sum
  let ribbon := dims.map (fun (w, l, h) =>
      let wrap := [2*(w+l), 2*(l+h), 2*(w+h)].min?.get!
      let bow := w*l*h
      wrap + bow
    ) |>.sum

  IO.println s!"Paper: {paper}, ribbon: {ribbon}"
