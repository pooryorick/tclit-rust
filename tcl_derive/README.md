The `tcl_derive` crate provides the proc-macro implementation for the `tcl` crate.

1. The `#[proc]` attribute to get a Rust `fn` ready for registering as a Tcl command.

2. The `tclfn!{}` macro to define a Rust `fn` and register it as a Tcl command.

3. The `tclosure!{}` macro to define a Rust closure and register it as a Tcl command.

See [`tcl` crate's doc](../tcl/README.md) for more.

# License

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

To license this program under other terms, contact the author(s) of the
program.

