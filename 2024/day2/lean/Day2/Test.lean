import Day2.Basic
import Day2.Input

private def testIsSafe (str: String) : Bool := (parseReports str)[0]!.isSafe
private def testIsSafeDampened (str: String) : Bool :=
  (!testIsSafe str) && (parseReports str)[0]!.isSafeDampened

#guard testIsSafe "1 2 3"
#guard testIsSafe "3 2 1"
#guard testIsSafe "1 3 4 6"
#guard testIsSafe "6 4 3 1"

#guard testIsSafe "0 3 6"
#guard testIsSafe "6 3 0"

#guard testIsSafeDampened "1 2 3 3"
#guard testIsSafeDampened "1 2 2 3"
#guard testIsSafeDampened "1 1 2 3"

#guard testIsSafeDampened "1 2 3 99"
#guard testIsSafeDampened "1 2 99 3"
#guard testIsSafeDampened "99 1 2 3"

#guard testIsSafeDampened "1 4 7 5 7 10"
#guard testIsSafeDampened "1 4 7 5 6 9"

#guard !testIsSafeDampened "1 2 3 2 1"
#guard !testIsSafeDampened "1 2 3 10 11 12"
#guard !testIsSafeDampened "0 4 8"
