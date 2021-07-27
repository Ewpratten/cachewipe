use std::{fs, path::{Path}};

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use failure::Error;
use gitignore::File;
use regex::Regex;
use walkdir::WalkDir;

fn should_delete(
    path: &Path,
    _gitignore: &File,
    re: &Regex,
    ignored_roots: &Vec<&str>,
) -> Result<bool, Error> {
    // Strip the prefix off the path
    let path = path.strip_prefix("./")?;

    Ok(
        // Path is not being ignored by a root rule
        !ignored_roots
        .iter()
        .any(|ignore| path.starts_with(ignore))
        
        // Path matches REGEX
        && re.is_match(path.to_str().unwrap()) 
        
        // Path is being gitignored
        // && gitignore.is_excluded(&Path::new("/").join(path))?
    )   
}

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("pattern")
                .takes_value(true)
                .help("File or path REGEX pattern to delete")
                .required(true),
        )
        .arg(
            Arg::with_name("dry_run")
                .long("dry-run")
                .takes_value(false)
                .required(false)
                .help("Just print file paths to stdout. Don't delete anything"),
        )
        .arg(
            Arg::with_name("root_ignores")
                .long("ignore-root-dir")
                .short("i")
                .help("Add a root-level directory to the list of dirs to ignore")
                .takes_value(true)
                .multiple(true)
                .required(false),
        )
        .get_matches();

    // Get data
    let pattern = matches.value_of("pattern").unwrap();
    let dry_run = matches.is_present("dry_run");
    let mut ignores = match matches.values_of("root_ignores") {
        Some(ignores) => ignores.collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    // DISALLOW an empty pattern
    if pattern.is_empty() {
        println!("You do NOT want an empty pattern!!!!");
        return;
    }

    // Always ignore .git
    ignores.push(".git");

    // Set up the regex
    let re = regex::Regex::new(pattern).unwrap();

    // Load the gitignore
    let gitignore = gitignore::File::new(Path::new(".gitignore"))
        .expect("Could not load '.gitignore'. Are you in the correct directory?");

    // Walk the directory structure
    for path in WalkDir::new(".") {
        match path {
            Ok(path) => {
                // Only run if the path is not ignored
                if should_delete(path.path(), &gitignore, &re, &ignores).unwrap_or(false) {
                    // Print the path
                    println!("DEL: {}", path.path().display());

                    // If not in dry run, delete it
                    if !dry_run {
                        fs::remove_file(path.path()).unwrap();
                    }             
                }
            }
            Err(_) => todo!(),
        }
    }
}
