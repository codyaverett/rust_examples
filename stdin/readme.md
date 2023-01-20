# Output

When passed text over stdin

```shell
~/rust_examples/stdin (main*) » echo "dumm dum" | cargo run                              
   Compiling libc v0.2.139
   Compiling atty v0.2.14
   Compiling stdin v0.1.0 (/Users/caavere/Projects/play/rust_examples/stdin)
    Finished dev [unoptimized + debuginfo] target(s) in 4.63s
     Running `target/debug/stdin`
line: dumm dum
```

When stdin is not redirected

```shell
~/rust_examples/stdin (main*) » cargo run                                                
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/stdin`
Error: Custom { kind: Other, error: "stdin not redirected" }
```