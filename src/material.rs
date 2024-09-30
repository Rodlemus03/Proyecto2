use crate::color::Color;
use image::{RgbaImage};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub textura: Option<Arc<RgbaImage>>, 
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 2], textura: Option<Arc<RgbaImage>>) -> Self {
        Self {diffuse,specular,albedo,textura,}
    }

    pub fn black() -> Self {
        Self::new(Color::new(0, 0, 0), 0.0, [0.0, 0.0], None)
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if let Some(ref textura) = self.textura {
            let (tex_width, tex_height) = (textura.width() as f32, textura.height() as f32);
            let (u, v) = (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0));
            let (x, y) = ((u * (tex_width - 1.0)).floor() as u32,(v * (tex_height - 1.0)).floor() as u32,);
            let pixel = textura.get_pixel(x.min(tex_width as u32 - 1), y.min(tex_height as u32 - 1));
            return Color::new(pixel[0], pixel[1], pixel[2]);
        }

        self.diffuse
    }
}
