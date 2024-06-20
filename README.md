international fixed calendar

\- [wikipedia](https://en.wikipedia.org/wiki/International_Fixed_Calendar) -

### crate features
**`local-offset`** - enables `Date::now_local()` to get the current IFC date in the local timezone.  
**`passtime`** - rexports the `time` crate.

### fica
a binary, with a very bad interface, to get the current IFC date in the
terminal. Also enables you to print the date in the gregorian calendar. Also
enables you to print the start, middle, and end of the month.
*(the middle is taken as the first day of the third week)*.

**flags** - pass as many of them you want in any order  
`fica greg` - get the gregorian date, too  
`fica start` - print the start of the month, too  
`fica middle` - print the middle of the month, too  
`fica end` - print the end of the month, too  