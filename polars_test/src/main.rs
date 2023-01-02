:dep polars = { version = "0.23.2", features = ["lazy", "csv-file", "strings", "temporal", "dtype-duration", "dtype-categorical", "concat_str", "list", "list_eval", "rank", "lazy_regex"]}
:dep color-eyre = {version = "0.6.2"}
:dep rand = {version = "0.8.5"}
:dep reqwest = { version = "0.11.11", features = ["blocking"]}

use color_eyre::{Result};
use polars::prelude::*;

fn main() {
    println!("Hello, world!");
}
