use image::{self, imageops, GenericImageView};

pub fn main() {
    let size = 128;
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: image_thumb <filename>");
        return;
    }

    let infile = String::from(&args[1]);
    let outfile = format!("{}-thumb.png", infile);
    println!("{} -> {}", infile, outfile);

    let mut img = image::open(&infile).expect("Failed to open image");
    let dim = img.dimensions();
    let w = if dim.0 > dim.1 { dim.1 } else { dim.0 };
    let mut img2 = imageops::crop(&mut img, (dim.0 - w) / 2, (dim.1 - w) / 2, w, w).to_image();
    let img3 = imageops::resize(&mut img2, size, size, imageops::Lanczos3);
    img3.save(outfile).unwrap();
}
