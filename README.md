# RaCNuL
RAndom Crazy NUmeration Language

## Getting started
### What is RaCNuL?
- RaCNuL is a crazy numeration system that looks like BrainFuck, but represent numbers.

## Mini RaCNul reference (not working yet!)
### Characters (CMDs)
- `+` Increment the number in current position
- `-` Negates the entire number
- `>` Go to the next position
- `<` Go to the last position
- `#` Create a new digit (default value 0)
- `,` Copy the number in the current position
- `.` Paste the copied number in the current position
- `%` Set the number in the current position to 0
### Examples

- Number -676
```
++++++ , > # +++++++ > # . -
^----- ^ ^ ^ ^------ ^ ^ ^ ^
A      B C D E       F G H I
```
   + A - Increment the current position by 6
   + B - Copy the number 6
   + C/F - Advance to the next position
   + D/G - Create the new position
   + E - Increment the current position by 7
   + H - Paste the number 6
   + I - Negates the number