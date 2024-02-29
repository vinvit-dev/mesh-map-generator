use image::io::Reader;

pub mod buffer;
pub mod camera;
pub mod keyboard_handler;
pub mod shader;
pub mod shader_program;

pub fn load_texture(path: &str) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let img = Reader::open(path).unwrap().decode().unwrap();
    img.to_rgba8()
}
