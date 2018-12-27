RPN
===

A simple Reverse-Polish Notation calculator written in rust with 128bit integer support.

Provides a simple command line interface. in addition to numbers and mathematical operands the system supports the following commands:

- **clear** clears registers and stack.
- **clmem** wipes everything including saved registers.
- **store** saves the current registers to memory.
- **recal** overwrites current registers with stored values.
- **sstk** prints the stack.
- **exit** duh
- **quit** same as exit.

Feature wishlist:

- [x] 5-function calc.
- [x] "infinite" stack.
- [x] X, Y, and Z registers.
- [x] Register saving (STORE).
- [x] Support for displaying current status including whether the stack is in use or values are stored.
- [ ] Scientific functions.
