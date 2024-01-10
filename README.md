# Reproducer for ...
1. Load the project to vscode.
1. `mkdir -p target`
1. `echo test > target/test.txt`
1. `cargo run -p consumer` outputs the contents of `Cargo.toml` and `target/test.txt`
1. Open [main.rs](consumer/src/main.rs)
1. Execute `rust-analyzer: Expand macro recursively at caret` at line 4. Contents of `Cargo.toml` is shown.
1. Move the cursor to line 5.
1. Execute the same action.

Expected behavior: The expansion should contain:
```
// Recursive expansion of generate_include_str! macro
// ===================================================

let contents = "abc";
{
  $crate::io::_print(builtin #format_args("{contents}"));
};
```

Actual behavior:
```
// Recursive expansion of generate_include_str! macro
// ===================================================

let contents = "";
{
  $crate::io::_print(builtin #format_args("{contents}"));
};
```
