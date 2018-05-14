use bmp::{Image,Pixel};

#[derive(Debug,Clone)]
pub struct Fractal {
    pub name: String,
    width: u32,
    height: u32,
    cx: f32,
    cy: f32,
    image: Image, 
}

impl Fractal {
    pub fn new(name: String, width: u32, height: u32, cx: f32, cy: f32) -> Fractal
    {
        Fractal {
            name, 
            width, 
            height, 
            cx, 
            cy,
            image: Image::new(width, height),
        }
    }

    pub fn set_all_pixels(&mut self)
    {
        for x in 0..self.width {
            for y in 0..self.height {
                let val = self.julia_pixel(x,y);
                self.image.set_pixel(x,y,Pixel::new(
                        (val << 3) as u8,
                        (val << 5) as u8,
                        (val << 4) as u8
                    ));
            }
        } 
    }

    //the algo used here is the julia fractal
    //the return value is a proportion on zero, we have to multiply it by our color space
    fn julia_pixel(&self, x: u32, y:u32) -> i32
    {
        let mut zx = 3.0 * (x as f32 - 0.5 * self.width as f32) / (self.width as f32);
        let mut zy = 2.0 * (y as f32 - 0.5 * self.height as f32) / (self.height as f32);

        let mut iter = 256;
        let max_iter = 256;

        while zx*zx + zy*zy < 4.0 && iter > 1 {
            let xtemp = zx*zx - zy*zy + self.cx;
            zy = 2.0*zx*zy + self.cy;
            zx = xtemp;

            iter -= 1;
        }
        iter
    }

    pub fn save(&self, path: String)
    {
        self.image.save(path);
    }
}
