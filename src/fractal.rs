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
                let val = (self.julia_pixel(x,y) * 256.0) as u8;
                self.image.set_pixel(x,y,Pixel::new(val,val,val));
            }
        } 
    }

    //the algo used here is the julia fractal
    //the return value is a proportion on zero, we have to multiply it by our color space
    fn julia_pixel(&self, x: u32, y:u32) -> f32
    {
        let mut iter = 1000;
        let max_iter = 1000.0;

        let mut zx = x as f32 / self.width as f32 * 3.5 - 2.5;
        let mut zy = y as f32 / self.height as f32 * 2.0 - 1.0;

        while zx*zx + zy*zy < 4.0 && iter > 0 {
            let xtemp = zx*zx -zy*zy;
            zy = 2.0*zx*zy + self.cy;
            zx = xtemp + self.cx;

            iter -= 1;
        }
        iter as f32/max_iter
    }

    pub fn save(&self, path: String)
    {
        self.image.save(path);
    }
}
