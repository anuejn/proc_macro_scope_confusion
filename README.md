# Rust proc-macro-scope-confusion

The code in this repository does not compile with the following error:
```sh
anuejn@pink ~/tmp/rust_bug main $ cargo run
   Compiling macros v0.1.0 (/home/anuejn/tmp/rust_bug/macros)
   Compiling rust_bug v0.1.0 (/home/anuejn/tmp/rust_bug)
warning: unused variable: `a`
  --> src/main.rs:5:24
   |
5  |         let _ = move | a : String | $normal_macro_input;
   |                        ^ help: if this is intentional, prefix it with an underscore: `_a`
...
12 |     normal_macro!(demo_proc_macro!());
   |     ---------------------------------- in this macro invocation
   |
   = note: `#[warn(unused_variables)]` on by default
   = note: this warning originates in the macro `normal_macro` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0382]: borrow of moved value: `a`
  --> src/main.rs:14:13
   |
5  |         let _ = move | a : String | $normal_macro_input;
   |                 ------------------- value moved into closure here
...
10 |     let a = "some string".to_owned();
   |         - move occurs because `a` has type `String`, which does not implement the `Copy` trait
11 | 
12 |     normal_macro!(demo_proc_macro!());
   |                   ------------------ variable moved due to use in closure
13 | 
14 |     let _ = a.clone();
   |             ^ value borrowed here after move

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
error: could not compile `rust_bug`

To learn more, run the command again with --verbose.
```

This means the `a` symbol emited by the `demo_proc_macro` has a confusing scope: it is not bound to
the expansion site of the proc macro but rather to the expansion side of the `normal_macro`.
