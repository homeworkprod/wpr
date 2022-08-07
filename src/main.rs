/*
 * Copyright 2020-2021 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args = parse_args();

    let images_path = args.value_of("images_path").map(Path::new).unwrap();
    ensure_directory(&images_path);

    let files = images_path
        .read_dir()
        .expect("Failed to read path.")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file());

    let images: Vec<PathBuf> = files.filter(|path| is_extension_relevant(path)).collect();

    if images.is_empty() {
        eprintln!(
            "No supported images found in directory: {}",
            images_path.display()
        );
        std::process::exit(66); // EX_NOINPUT
    }

    let random_image = get_random_image(&images).expect("No images found.");

    set_wallpaper(random_image);
}

fn parse_args() -> ArgMatches {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("images_path")
                .help("Specify directory with images")
                .required(true),
        )
        .get_matches()
}

fn ensure_directory(path: &Path) -> () {
    if !path.exists() {
        eprintln!("Path does not exist: {}", path.display());
        std::process::exit(66); // EX_NOINPUT
    }

    if !path.is_dir() {
        eprintln!("Path is not a directory: {}", path.display());
        std::process::exit(66); // EX_NOINPUT
    }
}

fn is_extension_relevant(path: &Path) -> bool {
    let relevant_extensions = HashSet::from(["gif", "jpeg", "jpg", "png"]);

    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| relevant_extensions.contains(ext))
        .unwrap_or(false)
}

fn get_random_image(images: &[PathBuf]) -> Option<&Path> {
    let mut rng = rand::thread_rng();
    images.choose(&mut rng).map(|p| p.as_path())
}

fn set_wallpaper(path: &Path) {
    let filename = path.to_str().unwrap();

    Command::new("feh")
        .arg("--bg-fill")
        .arg(filename)
        .status()
        .expect("Failed to execute feh.");
}
