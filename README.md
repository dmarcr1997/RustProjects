# RustProjects

This is a directory of [Rust][] projects 

## Dependencies

- [Rust][]
- [Cargo][]

## Projects

<details><summary><a href="./guessing_game">Guessing Game</a></summary><br/>

- Dependencies
    - Rand
    - Rust std

</details>
<details><summary><a href="./structs_example">Structs Example</a></summary><br/>

- Summary
    - Example of struct creation, usage, and debug logging

</details>
<details><summary><a href="./RustyPipeViewer">Rusty Pipe Viewer</a></summary><br/>

- Dependencies
    - crossterm
    - crossbeam
    - clap
    - Rust std

</details>
<details><summary><a href="./GrepProject">Grep Project</a></summary><br/>

- Dependencies
    - Rust std
- Running project
    - cargo run query file > output.txt
    - Example 
        - cargo run to ./data/poem.txt > output.txt
    - use env variable IGNORE_CASE=1 to ignore case in searh

</details>

----------------------------------------------------------------
## Build instructions

```sh
# Clone this repo
git clone https://github.com/dmarcr1997/RustProjects.git
cd	RustProjects/*project you want to run*

# Run
cargo run
```

## License

Author: Damon M. Rocha

This project is distributed under the terms of the MIT license
[&lt;LICENSE&gt;](LICENSE).



[Rust]: https://www.rust-lang.org/
[Cargo]: https://crates.io/
[Guessing Game]: ./guessing_game
