international fixed calendar

\- [wikipedia](https://en.wikipedia.org/wiki/International_Fixed_Calendar) -

doesn't try to keep a similar interface to the `time` crate, sorry. i wanted
an ifc crate i was happy using and i'm alright letting some saftey go for that
apparently *(will happily allow you to construct an invalid date for example)*

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