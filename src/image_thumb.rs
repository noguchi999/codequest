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
}
