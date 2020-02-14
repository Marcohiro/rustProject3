pub mod image{
    use std::fs::File;
    use std::path::Path;
    use std::io::Write;
    use std::io::{BufReader};
    use std::io::BufRead;

    use crate::pixel::pixel;

    //build an image
    pub struct Image {
        pub pixels: Vec<pixel::Pixel>,
        pub height:u32,
        pub width:u32,
        pub max_color_val:u32
    }

    impl Image{
 
        pub fn new_with_file(filename: &Path) -> Image{
            let file = File::open(filename).unwrap();
            let file2 = File::open(filename).unwrap();
            let buf_reader = BufReader::new(file);
            let buf_reader2 = BufReader::new(file2);
            let mut h = 0;
            let mut w = 0;
            let mut m = 0;
            let mut pixels: Vec<pixel::Pixel> = Vec::new();
            for (index, line) in buf_reader.lines().enumerate(){
                let line = line.unwrap();
                if index == 0{
                    //On ignore la premiere ligne, on suppose toujours P3
                }
                else if index == 1{
                    //On recupere le max width et height
                    let list: Vec<&str> = line.split(' ').collect();
                    h = list[0].parse().unwrap();
                    w = list[1].parse().unwrap();
                }
                else if index == 2{
                    //On recupere la valeur maximale de la valeur de couleur
                    m = line.parse().unwrap();
                }
                else if index > 2{
                    //On recupere les pixels
                    let list: Vec<&str> = line.split_whitespace().collect();
                    let r:u8 = list[0].parse().unwrap();
                    let g:u8 = list[1].parse().unwrap();
                    let b:u8 = list[2].parse().unwrap();
                    let pixel = pixel::Pixel::new(r, g, b);
                    pixels.push(pixel);
                }
            }     

            return Image{
                height: h,
                width: w,
                max_color_val: m,
                pixels: pixels,
            };
        }
    
        pub fn save(&self, filename: &Path) -> std::io::Result<()> {
            let mut file = File::create(filename)?;
            file.write("P3\n".as_bytes())?;
            file.write(self.height.to_string().as_bytes())?;
            file.write(" ".as_bytes())?;
            file.write(self.width.to_string().as_bytes())?;
            file.write("\n".as_bytes())?;
            file.write(self.max_color_val.to_string().as_bytes())?;
            file.write("\n".as_bytes())?;
            for i in 0..self.pixels.len() {
                file.write(self.pixels[i].get_red().to_string().as_bytes())?;
                file.write(" ".as_bytes())?;
                file.write(self.pixels[i].get_green().to_string().as_bytes())?;
                file.write(" ".as_bytes())?;
                file.write(self.pixels[i].get_blue().to_string().as_bytes())?;
                file.write("\n".as_bytes())?;
            }
             Ok(())
        }
    }

    pub fn invert(image: Image) -> Image{
        let max = image.max_color_val;
        let w = image.width;
        let h = image.height;
        let mut pixels = image.pixels;
        for i in 0..pixels.len(){
            pixels[i].invert();
        }
        return Image{
            height: h,
            width: w,
            pixels: pixels,
            max_color_val: max,
        };
    }

    pub fn grayscale(image: Image) -> Image{
        let max = image.max_color_val;
        let w = image.width;
        let h = image.height;
        let mut pixels = image.pixels;
        for i in 0..pixels.len(){
            pixels[i] = pixel::grayscale(pixels[i]);
        }
        return Image{
            height: h,
            width: w,
            pixels: pixels,
            max_color_val: max,
        };
    }
}

#[cfg(test)]
mod image_test{
    
    use std::path::Path;

    use crate::image::image;
    use crate::pixel::pixel;

    #[test]
    fn image_load(){

        let mut pixels : Vec<pixel::Pixel> = Vec::new();
        pixels.push(pixel::Pixel::new(255, 0, 0));
        pixels.push(pixel::Pixel::new(0, 255, 0));
        pixels.push(pixel::Pixel::new(0, 0, 255));
        pixels.push(pixel::Pixel::new(255, 255, 0));
        pixels.push(pixel::Pixel::new(255, 255, 255));
        pixels.push(pixel::Pixel::new(0, 0, 0));
        
        let image = image::Image::new_with_file(Path::new("./test.ppm"));
        assert_eq!(image.pixels.len(), 6);

        assert_eq!(image.height, 3);
        assert_eq!(image.width, 2);
        assert_eq!(image.max_color_val, 255);
        assert_eq!(image.pixels.len(), pixels.len());
        for i in 0..image.pixels.len() {
            assert_eq!(image.pixels[i] == pixels[i], true);
        }
    }

    #[test]
    fn image_save(){
        let image = image::Image::new_with_file(Path::new("./test.ppm"));
        let _result = image.save(Path::new("./result.ppm"));
        assert_eq!(1,1);
    }

    #[test]
    fn image_invert(){
        let mut pixels : Vec<pixel::Pixel> = Vec::new();
        pixels.push(pixel::Pixel::new(0, 255, 255));
        pixels.push(pixel::Pixel::new(255, 0, 255));
        pixels.push(pixel::Pixel::new(255, 255, 0));
        pixels.push(pixel::Pixel::new(0, 0, 255));
        pixels.push(pixel::Pixel::new(0, 0, 0));
        pixels.push(pixel::Pixel::new(255, 255, 255));

        let image = image::Image::new_with_file(Path::new("./test.ppm"));
        let inv = image::invert(image);
        assert_eq!(inv.height, 3);
        assert_eq!(inv.width, 2);
        assert_eq!(inv.max_color_val, 255);
        assert_eq!(inv.pixels.len(), pixels.len());
        for i in 0..inv.pixels.len() {
            assert_eq!(inv.pixels[i] == pixels[i], true);
        }
        let _result = inv.save(Path::new("./inverse.ppm"));
    }

    #[test]
    fn image_grayscale(){
        let mut pixels : Vec<pixel::Pixel> = Vec::new();
        pixels.push(pixel::Pixel::new(100, 100, 100));
        pixels.push(pixel::Pixel::new(100, 100, 100));
        pixels.push(pixel::Pixel::new(50, 50, 50));
        pixels.push(pixel::Pixel::new(150, 150, 150));
        pixels.push(pixel::Pixel::new(250, 250, 250));
        pixels.push(pixel::Pixel::new(0, 0, 0));
        let image = image::Image::new_with_file(Path::new("./test.ppm"));
        let gray = image::grayscale(image);
        assert_eq!(gray.height, 3);
        assert_eq!(gray.width, 2);
        assert_eq!(gray.max_color_val, 255);
        assert_eq!(gray.pixels.len(), pixels.len());
        for i in 0..gray.pixels.len() {
            assert_eq!(gray.pixels[i] == pixels[i], true);
        }
        let _result = gray.save(Path::new("./grayImage.ppm"));
    }
}