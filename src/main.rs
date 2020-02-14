use std::path::Path;
pub mod pixel;
pub mod image;

pub use crate::pixel::pixel as pixel_mod;
pub use crate::image::image as image_mod;

fn main(){
    let path = Path::new("test.ppm");
    let mut image = image_mod::Image::new_with_file(path);
}