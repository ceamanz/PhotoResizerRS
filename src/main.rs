
use std::{fs, io, path::{Path, PathBuf}, thread, time};

use image::{DynamicImage, ImageReader, RgbImage};

fn main() -> image::error::ImageResult<()> {
    let timenow = time::Instant::now();
    let mut files = Vec::new();
    let path = Path::new(".");
    walk_dir(path, &mut files);
    let number_files = files.len();
    let path = Path::new("_resized");
    let _ = fs::create_dir(path);
    let mut tasks = Vec::new();
    for file in files {
        let handle = thread::spawn(move || {export_photo(file, 1024)});
        tasks.push(handle);
    }
    for task in tasks {
        let _ = task.join();
    }
    println!("Total {} files are resized, time consumed: {:?}.", number_files, time::Instant::now() - timenow);
    println!("Resizing completed, press any key to quit. Thank you for using. -- Capt.Gu Yibin");
    let _ = io::stdin().read_line(&mut String::new());
    Ok(())
}

fn walk_dir(dir: &Path, mut res: &mut Vec<PathBuf>) {
    if dir.is_dir() {
        if dir.starts_with(".\\_resized") {
            return;
        }
        let root_dir = Path::new("_resized");
        let join_dir = root_dir.join(dir);
        let _ = fs::create_dir(join_dir);
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    walk_dir(&path, &mut res);
                } else {
                    if is_photo(&path) {
                        res.push(path);
                    }
                }
            }
        }
    }
}

fn export_photo(file: PathBuf, size: u32) -> image::error::ImageResult<()> {
    let root_dir = Path::new("_resized");
    let file_name = root_dir.join(file.clone());
    let img = ImageReader::open(&file)?;
    let img = img.decode()?;
    let img = img.thumbnail(size, size);
    let img = to_rgb8(img);
    img.save_with_format(file_name, image::ImageFormat::Jpeg)?;
    Ok(())
}

fn is_photo(file: &PathBuf) -> bool {
    let file = ImageReader::open(file).unwrap();
    match file.format() {
        Some(_) => {return true}
        None => {return false}
    }
}

fn to_rgb8(img: DynamicImage) -> RgbImage {
    match img {
        DynamicImage::ImageRgb8(out_image) => out_image,
        _ => img.to_rgb8(),
    }
}
