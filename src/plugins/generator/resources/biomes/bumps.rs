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
        let mut color = Color::srgb(generator.get_noise(pos.x), generator.get_noise(pos.y), generator.get_noise(pos.z));

        if pos.y + 0.5 - self.layer2d.grass_value < 0. && value < 0.003 {
            color = Color::srgb(generator.get_norm_noise(pos.y), generator.get_norm_noise(pos.z), generator.get_norm_noise(pos.x));
        }

        color
    }

    fn get_voxel_value(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let noise_v = if pos.y <= 2. {
            let t = generator.get_noise3(pos);
            if t < 0. {
                let t = generator.get_noise3(Vec3 { x: pos.y, y: pos.z, z: pos.x });
                if t < 0. {
                    generator.get_noise3(Vec3 { x: pos.y, y: pos.z, z: pos.x })
                }else{
                    t
                }
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
