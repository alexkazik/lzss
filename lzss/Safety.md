# Safety

With the `safe` feature the code is not using any unsafe code (`forbid(unsafe_code)`), but at
the cost of performance and size - though on modern systems that is not to mention.

But on smaller systems (like microcontrollers, where `no_std` is needed) it may be noticeable.
Which is the reason wht it can be switched on/off.

## unsafe
  - is only used in two cases
    - skip bounds check
    - cast `&vec![0u8;N]` to `&[u8;N]` (simplified example, only used in generic with heap)
  - can't panic

## safe
  - does bounds and cast checks
  - can panic (includes messages and code for it)

## Possible failure

The code should never do "illegal" things, and thus the usage of `unsafe` would be no risk,
as well as `safe` would never panic.

There are many tests to check for this.

But still with an error in the code the unsafe version could access out of bounds and the
safe version could panic.
