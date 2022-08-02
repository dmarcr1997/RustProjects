use transformer::{toml_dyn, toml_stat};

fn main() {
    toml_dyn::read_dynamic_toml();
    println!();
    toml_stat::read_toml_static();
}
