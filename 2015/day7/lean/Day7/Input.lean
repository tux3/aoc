import Day7.Basic

def Value.parse (str: String): Value :=
  if let some num := str.toNat? then
    Value.nat num
  else Value.ref str

def Gate.parse (str: String): Gate :=
  match str.splitOn " " with
  | [val] => Gate.op0 (Value.parse val)
  | [op, val] => Gate.op1 op (Value.parse val)
  | [lval, op, rval] => Gate.op2 op (Value.parse lval) (Value.parse rval)
  | _ => panic! "Unexpected number of elements in netlist gate definition"

def Netlist.parseFile (path: System.FilePath): IO Netlist := do
  let netlist := (← IO.FS.readFile path).trimRight.splitOn "\n" |>.map (fun line =>
    let elems := line.splitOn " -> "
    (elems[1]!, Gate.parse elems[0]!)
  )
  pure ⟨ Std.HashMap.ofList netlist ⟩
