use std::{env, error::Error, fs, io, path::PathBuf};

use crate::css_binder::CssBinder;

pub mod css_binder;

fn get_stylesheets_paths(dir_path: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut stylesheets: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(dir_path)? {
        let path = entry?.path();
        if path.is_dir() {
            stylesheets.append(&mut get_stylesheets_paths(
                path.display().to_string().as_str(),
            )?);
        } else if path.display().to_string().ends_with(".wal.css") {
            stylesheets.push(path);
        }
    }
    Ok(stylesheets)
}

/// Script for creating a single .css file based on .css and .wal.css files form a given directory
///
/// This script takes two arguemts:
/// 1) path to directory which will be scanned for .wal.css files (f.e. ../../styles)
/// 2) path where the file with bound styles will be created (f.e. ../../styles)
///
/// example call : cargo run -- ../../styles ../../styles

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 4 || args.len() < 3 {
        panic!("Program takes only 2 or 3 arguments.");
    }

    let dir_path = &args[1];
    let out_path = &args[2];

    let mut binder = CssBinder::new(out_path);
    if args.len() == 4 {
        let global_path = &args[3];
    }

    let stylesheets = get_stylesheets_paths(dir_path)?;
    for stylesheet_path in stylesheets {
        binder.bind_stylesheet(stylesheet_path).unwrap();
    }

    println!("Css sucessfully bound!");
    Ok(())
}
