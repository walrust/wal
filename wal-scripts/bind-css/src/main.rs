use std::{env, fs, io, path::PathBuf};

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_path = &args[1];
    let out_path = &args[2];

    let stylesheets = get_stylesheets_paths(dir_path).unwrap();

    let mut binder = CssBinder::new(out_path);

    for stylesheet_path in stylesheets {
        binder.bind_stylesheet(stylesheet_path).unwrap();
    }

    println!("Css sucessfully bound!")
}
