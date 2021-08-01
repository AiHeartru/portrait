extern crate image;
extern crate rand;

use std::path::Path;

use image::{ImageBuffer, Rgb, imageops::{self, FilterType}};
use rand::{Rng, thread_rng};

#[derive(Debug)]
enum Axis {
    Horizontal,
    Vertical
}

pub struct Portrait ();
impl Portrait {
    // 创建画布
    fn create_canvas(width: u32, height: u32, color: [u8; 3]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        println!("创建画布中...");
        ImageBuffer::from_pixel(width, height, Rgb(color))
    }

    // 创建图层
    fn create_layer(color: [u8; 3]) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        println!("创建画布完成");
        println!("创建图层中...");
        ImageBuffer::from_pixel(5, 5, Rgb(color))
    }

    // 生成
    pub fn gen_portrait(width: u32, height: u32, color: [u8; 3]) -> String {
        let mut canvas = Self::create_canvas(width, height, color);
        let mut layer = Self::create_layer(color);
        println!("创建图层完成");
        println!("生成中...");
        let mut rng = thread_rng();
        let axis = if rand::random::<bool>() {
            Axis::Horizontal
        } else {
            Axis::Vertical
        };
        let color: [u8; 3] = rng.gen();
        match axis {
            Axis::Horizontal => {
                for i in 0..5 {
                    let p_y = rng.gen_range(0..5);
                    layer.put_pixel(i, p_y, Rgb(color));
                    layer.put_pixel(4 - i, p_y, Rgb(color));
                }
            },
            Axis::Vertical => {
                for i in 0..5 {
                    let p_x = rng.gen_range(0..5);
                    layer.put_pixel(p_x, i, Rgb(color));
                    layer.put_pixel(4 - p_x, i, Rgb(color));
                }
            }
        }
        let layer = imageops::resize(&layer, 128, 128, FilterType::Nearest);
        imageops::overlay(&mut canvas, &layer, 11, 11);
        let file_name = color.to_vec().iter().map(|x| format!("{:02x}", x)).collect::<String>() + ".png";
        let path = Path::new("static/").join(&file_name);
        canvas.save(&path).unwrap();
        println!("生成完成！");
        file_name
    }
}