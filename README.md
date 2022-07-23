# RaCNuL
RAndom Crazy NUmeration Language

## Getting started
### What is RaCNuL?
- RaCNuL is a crazy numeration system that looks like BrainFuck, but represent numbers.

## RaCNul reference
### Characters
- `+` Increment the number in current position
- `>` Go to the next position
- `<` Go to the last position
- `#` Create a new digit (default value 0)
- `,` Copy the number in the current position
- `.` Paste the copied number in the current position
- `%` Set the number in the current position to 0
### Examples
- Number 676 (non-optimized mode)
   ```
   ++++++ # > +++++++ # > ++++++
   ^----- ^ ^ ^------ ^ ^ ^-----
   A      B C D       E F G     
   ```
   + A - Increment the current position by 6
   + B/E - Create the new position
   + C/F - Advance to the next position
   + D - Increment the current position by 7
   + G - Increment the current position by 6
- Number 676 (optimized mode)
   ```
   ++++++ , # > . + # > .
   ^----- ^ ^ ^ ^ ^ ^ ^ ^
   A      B C D E F G H I
   ```
   + A - Increment the current position by 6
   + B - Copy the number 6
   + C/G - Create the new position
   + D/H - Advance to the next position
   + E - Paste the number 6
   + F - Increment the current position by 6 (turning it 7)
   + I - Paste the number 6

> By marc-dantas