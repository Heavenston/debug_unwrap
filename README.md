# debug_unwrap

Adds the method debug_unwrap for when you just want to make it compile.
Not to be confused with the <https://crates.io/crates/debug_unwraps> crate.

This library adds the [DebugUnwrap](DebugUnwrap) extension trait that adds
the method [debug_unwrap](DebugUnwrap::debug_unwrap) to the
[Option](std::option::Option) and [Result](std::result::Result) types.
It does exactly the same thing as the normal unwrap methods, but won't exist
when compiling without debug_assertions enabled
(i.e. when not compiling in Debug mode).

There is also three other aliases that all have there respective
library features of the same name to enable them:
 - out (enabled by default)
 - o
 - peel
