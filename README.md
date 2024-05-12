# RaCNuL
**RA**ndom and **C**razy **NU**mber **L**anguage is a number notation that looks like BrainFuck, but it's not a programming language. It was made with no purpose at all, just for fun.

## Getting started
This is a very simple tool that you can play with this notation. It's written in Rust.

- Running RaCNuL with `cargo`:
  ```console
  $ cd path/to/racnul
  $ cargo run
  ```

## Usage
You can use RaCNuL in 2 different ways:

- Via the Command Line (example):
  ```console
  $ racnul "++#>++"
  22
  ```

- Via the REPL (example):
  ```console
  $ racnul
  racnul> +++++++++
  output> 9
  cursor> ^
  ```
> **NOTE**: Type `exit` to quit the REPL

## Reference

| **Operation** | **Description**                             |
| ------------- | ------------------------------------------- |
| `+`           | Increment the number in current position    |
| `>`           | Go to the next position                     |
| `<`           | Go to the last position                     |
| `#`           | Create a new digit (default value 0)        |
| `,`           | Copy the number in the current position     |
| `.`           | Paste clipboard in current position         |
| `%`           | Set the number in the current position to 0 |

## Examples
- Number 676 (non-optimized)
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
- Number 676 (optimized)
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

---

> By marc-dantas