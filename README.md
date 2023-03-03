# crustdg

Rust crate for parsing CDG data

With little/incomplete documentation available parser accuracy/efficacy is suspect until more test files can be validated against.

Based on the following sources:

* [CD+G Revealed](https://jbum.com//cdg_revealed.html)
* [Tech Flashback: The CD+Graphics Format (CD+G)](https://goughlui.com/2019/03/31/tech-flashback-the-cdgraphics-format-cdg/)

## Using crustdg As A Dependency

```shell
$ cargo add --git "https://github.com/buckynbrocko/crustdg"
```

or

```toml
# Cargo.toml
[dependencies]
crustdg = { git = "https://github.com/buckynbrocko/crustdg", branch = "main"}
```

## TODOs

* [x] Implement Parser
* [ ] Convenience functions
* [ ] Create test files to verify correctness
* [ ] ...
