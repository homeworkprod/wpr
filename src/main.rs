use lazy_static::lazy_static;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

lazy_static! {
    static ref FILE_EXTENSIONS: HashSet<&'static str> =
        ["gif", "jpeg", "jpg", "png"].iter().cloned().collect();
}

fn main() {
    let path_str = parse_args();
    let path = ensure_directory(&path_str);

    let files = path
        .read_dir()
        .expect("Failed to read path.")
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file());

    let images: Vec<PathBuf> = files.filter(|path| is_extension_relevant(path)).collect();

    if images.is_empty() {
        eprintln!("No supported images found in directory: {}", path_str);
        std::process::exit(66); // EX_NOINPUT
    }

    let random_image = get_random_image(&images).expect("No images found.");

    set_wallpaper(random_image);
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!(
            "Wallpaper Randomizer. Sets a wallpaper (using feh) randomly chosen from a directory."
        );
        eprintln!("Usage: wpr <path>");
        std::process::exit(64); // EX_USAGE
    }

    args[1].to_string()
}

fn ensure_directory(path_str: &String) -> &Path {
    let path = Path::new(path_str);

    if !path.exists() {
        eprintln!("Path does not exist: {}", path_str);
        std::process::exit(66); // EX_NOINPUT
    }

    if !path.is_dir() {
        eprintln!("Path is not a directory: {}", path_str);
        std::process::exit(66); // EX_NOINPUT
    }

    path
}

fn is_extension_relevant(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| FILE_EXTENSIONS.contains(ext))
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
