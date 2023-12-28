use image::{DynamicImage, RgbaImage, GenericImageView, Pixel, Luma, ImageBuffer};

pub fn negative(img : &DynamicImage) -> RgbaImage{
    let (x,y) = img.dimensions();  
    let mut out_img = RgbaImage::new(x,y);
    for h in 0..y{
        for w in 0..x{
            let mut pix = img.get_pixel(w, h);
            for i in 0..3{
                pix[i] = 255 as u8 - pix[i];
            }
            out_img.put_pixel(w,h, pix);
        }
    }
    return out_img;
}

pub fn gray_img(img : &DynamicImage) ->ImageBuffer<Luma<u8>, Vec<u8>>{
    let (x,y) = img.dimensions();  
    let mut out_img = ImageBuffer::new(x,y);
    for h in 0..y{
        for w in 0..x{
            let pix = img.get_pixel(w, h).to_rgb(); 
            let mut tmp: u32 = 0 ;
            for i in 0..3{
                tmp = tmp +  pix[i] as u32;
            }
            let pixel: u8 = (tmp/3) as u8;
            out_img.put_pixel(w,h, image::Luma([pixel]));
        }
    }
    return out_img;
}

pub fn binary_img(img: &DynamicImage,threshold : i32) -> ImageBuffer<Luma<u8>, Vec<u8>>{
    let (x,y) = img.dimensions();  
    let mut out_img = ImageBuffer::new(x,y);
    let gray_img = gray_img(img);
    for h in 0..y{
        for w in 0..x{
            let pix = gray_img.get_pixel(w, h);
            if pix[0] >= threshold as u8{
                out_img.put_pixel(w,h,image::Luma([255]));
                continue;
            }
            out_img.put_pixel(w,h,image::Luma([0]));
        }
    }
    println!("{:?}",out_img);
    return out_img;
}