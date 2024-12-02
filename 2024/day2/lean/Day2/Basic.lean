structure Report where
  levels : List Int
deriving Repr

instance : Inhabited Report where
  default := Report.mk []

private def isSafeStep (asc : Bool) (a : Int) (b : Int) : Bool :=
  ((b - a).natAbs <= 3) && ((asc && a < b) || (!asc && a > b))

namespace Report
def isMainlyAsc (r : Report) : Bool :=
  let rec ascDescCounts : List Int → Int × Int
    | x1 :: x2 :: xs =>
      let (ascCount, descCount) := ascDescCounts (x2 :: xs)
      if x2 > x1 then (1 + ascCount, descCount)
      else if x1 > x2 then (ascCount, 1 + descCount)
      else (ascCount, descCount)
    | _ => (0, 0)
  let (ascCount, descCount) := ascDescCounts r.levels
  ascCount > descCount

private def isSafe_levels (isAsc: Bool) (levels: List Int) : Bool := match levels with
    | a :: b :: xs => (isSafeStep isAsc a b) && isSafe_levels isAsc (b :: xs)
    | _ => true

def isSafe (r : Report) : Bool := isSafe_levels r.isMainlyAsc r.levels

-- NOTE: This never tries to remove the first element, only from the 2nd onward
private def isSafeDampened_levels (isAsc: Bool) (levels: List Int) : Bool := match levels with
    | a :: b :: c :: xs =>
      ((isSafeStep isAsc a b) && (isSafeDampened_levels isAsc (b :: c :: xs)))
      || ((isSafeStep isAsc a c) && isSafe_levels isAsc (c :: xs))
    | _ => true

def isSafeDampened (r : Report) : Bool :=
  isSafeDampened_levels r.isMainlyAsc r.levels
  || isSafe_levels r.isMainlyAsc r.levels.tail -- Here we handle trying to skip the 1st

end Report
