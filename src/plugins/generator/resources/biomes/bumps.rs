use bevy::{math::Vec3, prelude::*};

use crate::plugins::{
    chunk::resources::chunk::object::Object, generator::resources::noise_generator::NoiseGenerator,
};

use super::BiomeGenerator;

#[derive(Default)]
pub struct Bumps2dLayer {
    grass_value: f32,
}

pub struct BumpsBiomeGenerator {
    layer2d: Bumps2dLayer,
}

impl BiomeGenerator for BumpsBiomeGenerator {
    fn update_2d_layer(&mut self, generator: &NoiseGenerator, pos: Vec2) {
        let grass_value = generator.get_noise2(pos * 100.);

        self.layer2d = Bumps2dLayer { grass_value }
    }

    fn get_voxel_color(&self, generator: &NoiseGenerator, pos: Vec3, value: f32) -> Color {
        if self.layer2d.grass_value - (generator.get_noise(pos.y) * 0.75) < generator.get_noise(pos.y) || pos.y > 1.45 {
            //GREEN
            Color::srgb(generator.get_noise(pos.y).clamp(50., 150.), (230. * 0.6) + (generator.get_noise(pos.x).clamp(100., 240.)*0.2) + (generator.get_noise(pos.z).clamp(100., 240.)*0.2), 0.)
        }else{
            let g = (generator.get_noise(pos.y).clamp(100., 240.) * 0.25) + (generator.get_noise(pos.x).clamp(100., 240.)*0.25) + (generator.get_noise(pos.z).clamp(100., 240.)*0.25);
            let r = g * 1.1;
            let b = g * 0.9;
            Color::srgb(r, g + 25., b)
        }
    }

    fn get_voxel_value(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let noise_v = if pos.y <= 1.5 {
            let t = generator.get_noise3(pos);
            if t < 0. {
                t + generator.get_noise3(Vec3 { x: pos.y, y: pos.z, z: pos.y }).abs()
            }else{
                t
            }
        }else{
            f32::MIN
        };
        noise_v
    }

    fn try_generate_object(&self, _pos: Vec3, _value: f32) -> Option<Object> {
        None
    }
}

impl BumpsBiomeGenerator {
    pub fn new() -> Self {
        Self {
            layer2d: Bumps2dLayer::default(),
        }
    }
}
