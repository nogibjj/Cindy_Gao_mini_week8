[![Rust CI/CD Pipeline](https://github.com/nogibjj/Cindy_Gao_week7_rust_template/actions/workflows/rust.yml/badge.svg)](https://github.com/nogibjj/Cindy_Gao_week7_rust_template/actions/workflows/rust.yml)

# Mini Project 7: Package a Python Script into a Command-Line Tool (or Rust)
## Requirements:<br>
* Package a Python script with setuptools or a similar tool
* Include a user guide on how to install and use the tool
* Include communication with an external or internal database (NoSQL, SQL, etc) [If you use Rust you can skip the DB part]


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

## File structure
```plaintext
Cindy_Gao_week7_rust_template/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .github/
│   └── workflows/
│       └── rust.yml
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── Makefile
├── README.md
├── user_guide.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   └── other_module.rs
├── target/
│   ├── debug/
│   ├── release/
│   └── ...
└── tests/
    ├── integration_test.rs
    └── my_tests.rs
```
* Inside the main.rs, there is a function called exponent which contains 2 parameters. The function is to calculate a to the power of b.
