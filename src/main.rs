mod error;

use std::env;
use std::fs::{read_to_string};
use std::path::{PathBuf};
use tree_sitter::{Parser};
use crate::error::AppError;

fn load() -> Result<String, AppError> {
    let args: Vec<String> = env::args().collect();

    let path = {
        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
        root.push(&args[1]);
        println!("{:?}", root);
        root
    };

    Ok(read_to_string(path.as_path())?)
}

fn main() -> Result<(), AppError> {
    let code = load()?;

    let mut parser = Parser::new();
    let language = tree_sitter_javascript::language();
    parser.set_language(language)?;

    let tree = match parser.parse(code, None) {
        Some(tree) => tree,
        None => {
            return Ok(());
        }
    };

    println!("{:?}", tree);

    Ok(())
}
