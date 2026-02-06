international fixed calendar

\- [wikipedia](https://en.wikipedia.org/wiki/International_Fixed_Calendar) -

doesn't try to keep a similar interface to the `time` crate, sorry. i wanted
an ifc crate i was happy using and i'm alright letting some saftey go for that
apparently *(will happily allow you to construct an invalid date for example)*

### crate features
**`local-offset`** - enables `Date::now_local()` to get the current IFC date in the local timezone.  
**`passtime`** - rexports the `time` crate.

### fica
a binary, with an *alright* interface, to get the current IFC date in the
terminal. it also lets you get the month boundaries, and month midpoint,
of an IFC month in the gregorian calendar.  
*(the month midpoint is taken as the first day of the third week, the 15th day)*

**examples**  
`fica` - get the current date in the IFC  
`fica elapsed` - to print the currently elapsed weeks, the current week, and their gregorian boundaries  
`fica month` OR `fica current` - get the boundaries of the current month  
`fica <month>` - get the boundaries of the *\<month\>* month