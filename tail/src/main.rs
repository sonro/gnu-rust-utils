use clap::Clap;
use std::fs;
use std::io::prelude::*;
// use std::path::PathBuf;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "sonro <sonro@gmx.com>")]
struct Opts {
    filepaths: Vec<String>,

    #[clap(short = 'n', default_value = "10")]
    lines: usize,
}

fn main() {
    let opts = Opts::parse();

    for path in &opts.filepaths {
        let metadata = fs::metadata(&path);
        if metadata.is_err() {
            continue;
        }
        let metadata = metadata.unwrap();
        if metadata.is_dir() {
            println!("==> {} <==", &path);
            println!("tail: error reading '{}': Is a directory", &path);
            continue;
        } else {
            let mut file = fs::File::open(&path).unwrap();
            let mut contents = String::with_capacity(metadata.len() as usize);
            if opts.filepaths.len() > 1 {
                println!("==> {} <==", &path)
            };
            file.read_to_string(&mut contents).unwrap();
            let lines = contents.lines();
            let nlines = lines.clone().count();
            if nlines <= opts.lines {
                print!("{}", contents);
            } else {
                let first_line = nlines - opts.lines;
                &lines.skip(first_line).for_each(|l| println!("{}", l));
            }
        }
    }
}
