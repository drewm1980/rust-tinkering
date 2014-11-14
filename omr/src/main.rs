#![crate_name = "music_ocr"]
#![crate_type = "rlib"]

extern crate png;
//use png::{Image,RGB8,store_png,load_rgba8,};
use png::{load_png,Image};

//use std::mem;
//use std::io;
//use std::io::File;
//use std::ptr;
//use std::slice;

#[cfg(test)]
mod test {
	extern crate test;
	use super::png::{load_png,Image};

#[test]
	fn test_easyload() {
		let loadfile = "sheet_music/La_yumba1.png";
		let loadpath = &Path::new(loadfile);
		let img = match load_png(loadpath){
			Ok(img) => img,
			Err(e) => panic!("Could not ooopen file: {}",loadfile)
};

		println!("Image width is {}",img.width);
		
		//let storefile = "La_yumba1_modified.png";
	}


//#[test]
	//fn test_store() {
		//let mut img = Image {
//width: 10,
	       //height: 10,
	       //pixels: RGB8(Vec::from_elem(10 * 10 * 3, 100u8)),
		//};
		//let res = store_png(&mut img, &Path::new("test/store.png"));
		//assert!(res.is_ok());
	//}
//#[test]
	//fn test_load() 
	//{
		//load_rgba8("test/servo-screenshot.png", 831, 624);
		//load_rgba8("test/store.png", 10, 10);
	//}
}

fn main() {
    println!("Hello, world!")
}
