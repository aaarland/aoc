use image::DynamicImage;

pub enum Rotation {
    North,
    East,
    South,
    West,
}

pub trait Sprite {
    fn image(&self) -> &DynamicImage;
}

pub fn load_sprite(
    path: &str,
    size: u32,
    rows: u32,
    cols: u32,
) -> Result<Vec<DynamicImage>, image::ImageError> {
    let sprite_sheet = image::open(path)?;
    let mut sprites = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let sprite = sprite_sheet.clone();
            let sprite = sprite.crop_imm(col * size, row * size, size, size);
            sprites.push(
                sprite,
            );
        }
    }
    Ok(sprites)
}
