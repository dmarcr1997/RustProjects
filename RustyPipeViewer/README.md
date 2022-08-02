# RustyPipeViewer

This is an educational version of the pv command line applcation
## Dependencies

- [Rust][]
- [Cargo][]
- [crossterm][]
- [crossbeam][]
- [clap][]

## Build/Run instructions

```sh
# Clone this repo
git clone https://github.com/dmarcr1997/RustProjects.
cd	RustProjects/RustyPipeViewer

# Run Examples

echo hello | cargo run
yes | cargo run -- -o /dev/null
cargo run -- file.txt -o file2.txt

# Command line options
-- input file -o outputfile 
-s for silent output
```

## License

Author: Damon M. Rocha

This project is distributed under the terms of the MIT license
[&lt;LICENSE&gt;](LICENSE).



[Rust]: https://www.rust-lang.org/
[Cargo]: https://crates.io/
[crossterm]: https://docs.rs/crossterm/latest/crossterm/
[crossbeam]: https://docs.rs/crossbeam/latest/crossbeam/
[clap]: https://docs.rs/clap/latest/clap/
