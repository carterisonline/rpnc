# RPNC - the Reverse Polish Notation Calculator

The work-in-progress ANSI-compliant terminal calculator replacement for  `dc` and `dci`

## Why use RPNC?

RPNC uses a method called [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation), which can be seen implemented in [Forth](https://en.wikipedia.org/wiki/Forth_(programming_language)) and Early HP Desktop Calculators starting back in the late '60s. It allows the user to think less about the placement of their symbols, and get to calculating stuff quicker.

> Yeah, but I don't have an issue with 1 + 1 = 2. What gives?

The simplicity of Reverse Polish Notation allows the user to reuse previous results easily. On several different Texas Instruments machines including the TI-83, this is often used as the `ANS` key.

The above formula would be written as `1 1 +` in Reverse Polish Notation, leaving the resulting `2` on the stack, ready to use with, say, `1 +`, which would add 1 to the stack, add 2 and 1 together, and push the result back (leaving 3, of course)

For more complex problems, Reverse Polish Notation makes this process even easier, without relying on the calculator to auto-assume `ANS` variable placement and such.

## What's planned for RPNC?

Currently, RPNC aims to be _almost_ like Vim. Most of these features aren't added yet, but will be added in future versions.

- Internal sub-stacks and operator borrowing
- Variable access and pointer operations
- Scriptable functions
- Lambda/Closure notation
- Scientific and Physics modes

## How to Build

Simply make sure that you have `rustup` installed, then run the following for an optimized build:

(first, make sure you have `strip` installed)

```sh
# Make sure you have `strip` installed
apt install binutils  # Debian/Ubuntu
apk add binutils      # Alpine
pacman -S binutils    # Arch
yum install binutils  # Centos
dnf install binutils  # Fedora
brew install binutils # macOS
```

Then run the following:

```sh
cargo install xargo &&
cargo install cargo-make &&
cargo make build
```
