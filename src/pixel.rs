pub mod pixel {
    //build a pixel
    
        pub struct Pixel {
            r:u8,
            g:u8,
            b:u8
        }
    
        impl Pixel{
            
            pub fn new(r:u8, g:u8, b:u8) -> Pixel{
                Pixel {
                    r:r,
                    g:g,
                    b:b,
                }    
            }
    
            pub fn get_red(self) -> u8{
                return self.r;
            }
    
            pub fn get_green(self) -> u8{
                return self.g;
            }
    
            pub fn get_blue(self) -> u8{
                return self.b;
            }
    
            pub fn display(self) -> String {
                let space = " ";
                let mut build_str = String::from("");
                let red = self.r.to_string();
                let green = self.g.to_string();
                let bleu = self.b.to_string();
                build_str.push_str(&red);
                build_str.push_str(space);
                build_str.push_str(&green);
                build_str.push_str(space);
                build_str.push_str(&bleu);
                return build_str;   
            }
    
            pub fn invert(&mut self){
                self.r = 255-self.r;
                self.g = 255-self.g;
                self.b = 255-self.b;
            }
    
        }
    
        impl Copy for Pixel { }
    
        impl Clone for Pixel {
            fn clone(&self) -> Pixel {
                *self
            }
        }
    
        pub fn grayscale(pixel: Pixel) -> Pixel{
            let value = ((pixel.get_red() as u32 + pixel.get_green() as u32 + pixel.get_blue() as u32) / 3) as u8;
            return Pixel::new(value, value, value);
        }
    
        impl PartialEq for Pixel {
            fn eq(&self, _other: &Self) -> bool {
                return self.r == self.r && self.g == self.g && self.b == self.b;
            }
        }
    }
    
    #[cfg(test)]
    mod pixel_tests {
        
        pub use super::pixel;
    
        #[test]
        fn new_pixel(){
            let pixel = pixel::Pixel::new(255, 255, 255);
            assert_eq!(pixel.get_red(), 255);
            assert_eq!(pixel.get_green(), 255);
            assert_eq!(pixel.get_blue(), 255);
        }
    
        #[test]
        fn pixel_display(){
            let pixel = pixel::Pixel::new(255, 255, 255);
            let value = pixel.display();
            assert_eq!("255 255 255", value);
        }
    
        #[test]
        fn pixel_invert(){
            let mut pixel = pixel::Pixel::new(200, 100, 50);
            pixel.invert();
            assert_eq!(pixel.get_red(), 55);
            assert_eq!(pixel.get_green(), 155);
            assert_eq!(pixel.get_blue(), 205);
        }
    
        #[test]
        fn pixel_partial_eq(){
            let pixel1 = pixel::Pixel::new(255, 255, 255);
            let pixel2 = pixel::Pixel::new(255, 255, 255);
            assert_eq!(pixel1 == pixel2, true);
            assert_eq!(pixel1 != pixel2, false);
        }
    
        #[test]
        fn pixel_clone(){
            let pixel = pixel::Pixel::new(10, 50, 100);
            let clone = pixel.clone();
            assert_eq!(pixel == clone, true);
        }
    
        #[test]
        fn pixel_grayscale(){
            let pixel = pixel::Pixel::new(200, 100, 30);
            let gray_pixel = pixel::grayscale(pixel);
            assert_eq!(gray_pixel.get_green(), gray_pixel.get_green());
            assert_eq!(gray_pixel.get_red(), gray_pixel.get_blue());
            assert_eq!(gray_pixel.get_green(), gray_pixel.get_blue());
            assert_eq!(gray_pixel.get_red(), 110);
        }
    
    }