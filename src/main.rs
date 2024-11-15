use image::{EncodableLayout, ImageBuffer, Rgba};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

const WIDTH: u32 = 50;
const HEIGHT: u32 = 50;

fn main() {
    let mut state = HashMap::new();

    for _ in 0..20 {
        let img_buffer = gen_image();

        let mut hasher = Sha256::new();

        hasher.update(img_buffer.as_bytes());

        let hash = format!("{:x}", hasher.finalize());

        state.insert(hash, img_buffer.into_raw());
    }

    for (hash, value) in state.iter() {
        let img_buffer =
            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(WIDTH, HEIGHT, value.clone()).unwrap();

        save_image(img_buffer, hash.clone());
    }
}

fn gen_image() -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut rng = rand::thread_rng();

    let mut image_data = Vec::with_capacity(WIDTH as usize * HEIGHT as usize * 4);

    for _ in 0..(WIDTH * HEIGHT) {
        let r = rng.gen_range(0..=255);
        let g = rng.gen_range(0..=255);
        let b = rng.gen_range(0..=255);
        let a = 255;

        image_data.push(r);
        image_data.push(g);
        image_data.push(b);
        image_data.push(a);
    }

    let img_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(WIDTH, HEIGHT, image_data)
        .expect("Error al crear el buffer de imagen");

    img_buffer
}

fn save_image(img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>, name: String) {
    img_buffer
        .save(format!("{}.png", name))
        .expect("Error al guardar la imagen PNG");
    println!("Archivo '{}.png' creado exitosamente.", name);
}
