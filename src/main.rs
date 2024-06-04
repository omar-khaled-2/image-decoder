extern crate bitstream_io;

use bitstream_io::{BigEndian, BitRead, BitReader, LittleEndian};
use std::env;
use std::fs::File;
use image::{ImageBuffer, Rgb, RgbImage};


struct PrefixTreeNode {

    children: (Option<Box<PrefixTreeNode>>,Option<Box<PrefixTreeNode>>),

    value:Option<(u8, u8, u8)>


}

impl PrefixTreeNode {
    fn new() -> Self {
        PrefixTreeNode {
            children: (None, None),
            value: None,
        }
    }
}




fn main() {

    let mut root = PrefixTreeNode::new();

    let mut image_path = String::new();

    let args: Vec<String> = env::args().collect();


    for i in 1..args.len() {

        if args[i] == "--path" {
            image_path = args[i + 1].to_string();
        }

    }


    let file = File::open(&image_path).unwrap();


    let mut reader = BitReader::endian(file, BigEndian);
    
    let width = reader.read::<u32>(16).unwrap();
    let height = reader.read::<u32>(16).unwrap();
    

    let k = reader.read::<u8>(8).unwrap();




    for _ in 0..k {
        let length = reader.read::<u8>(4).unwrap();
    
        let mut node = &mut root;
        

        for _ in 0..length {
            let code = reader.read_bit().unwrap();
    
            if code {
                
                if node.children.1.is_none() {
                    node.children.1 = Some(Box::new(PrefixTreeNode::new()));
                }

                node = node.children.1.as_mut().unwrap();
            }else{
                if node.children.0.is_none() {
                    node.children.0 = Some(Box::new(PrefixTreeNode::new()));
                }

                node = node.children.0.as_mut().unwrap();
            }
        }



        let r = reader.read::<u8>(8).unwrap();
        let g = reader.read::<u8>(8).unwrap();
        let b = reader.read::<u8>(8).unwrap();
        node.value = Some((r, g, b));
    }


    let mut img = RgbImage::new(width, height);




    for y in 0..height {
        for x in 0..width {
            let mut node = &root;
         
            loop{
            
                let code = reader.read_bit().unwrap();


    
                if code { 
            
                    node = node.children.1.as_ref().unwrap();
                }else{
                
                    node = node.children.0.as_ref().unwrap();
                }




                if node.value.is_some() {
             
                    let (r, g, b) = node.value.unwrap();
    
                    img.put_pixel(x, y, Rgb([r, g, b]));
                    break;
                }





            }


        }
    }


    img.save("image.jpg").unwrap();









}

