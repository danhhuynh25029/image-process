mod process;
use image::GenericImageView;

use process::operations::negative;

use crate::process::operations::{gray_img, binary_img};
fn main(){
    let img = image::open("/Users/danhhnc/Desktop/danhhuynh250/image-process/test.jpg").unwrap();
    println!("{:?}", img.color());
    println!("{:?}",img.dimensions());
    let negative_img = negative(&img);
    negative_img.save("negative.png").unwrap();

    let gray_img = gray_img(&img);
    gray_img.save("gray.png").unwrap();

    let binary_img = binary_img(&img, 120);
    binary_img.save("binary.png").unwrap();
}
