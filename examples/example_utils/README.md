
Because `cargo package` does not provide a means to ignore private crates, we opt instead to
use a module, which is imported into each example.

Because each example compiles separately, warnings are generated for each example that does
not make exhaustive use of the shared code.

When `cargo package` is capable of ignoring private crates that are `[dev-dependencies]`
then `example_utils` should be converted to a crate.