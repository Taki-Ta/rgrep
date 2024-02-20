### This is a program written while learning rust. Query content and query files support the use of regular expressions. 

```powershell
ps:rgrep impl\w+ test[1-9].rs
test1.rc
 3:0 impl Solution {
test2.rc
 1:0 impl Solution {
```

Above I want to search for "impl\w+" in the test1-9.rc file, and the program returns two search results, namely test1.rc and test2.rc. Among them, 3:0 respectively represent the search string appearing in the 0th character of the third line.