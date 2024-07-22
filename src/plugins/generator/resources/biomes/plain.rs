use bevy::{math::Vec3, prelude::*};

use crate::plugins::{
    chunk::resources::chunk::object::Object, generator::resources::noise_generator::NoiseGenerator,
};

use super::BiomeGenerator;

#[derive(Default)]
pub struct Plain2dLayer {
    grass_value: f32,
}

pub struct PlainBiomeGenerator {
    layer2d: Plain2dLayer,
}

impl BiomeGenerator for PlainBiomeGenerator {
    fn update_2d_layer(&mut self, generator: &NoiseGenerator, pos: Vec2) {
        let grass_value = generator.get_noise2(pos * 100.);

        self.layer2d = Plain2dLayer { grass_value }
    }

    fn get_voxel_color(&self, _generator: &NoiseGenerator, pos: Vec3, value: f32) -> Color {
        let mut color = Color::srgb(0.3, 0.3, 0.4);

        if pos.y + 0.5 - self.layer2d.grass_value < 0. && value < 0.003 {
            color = Color::srgb(0.1, 1.0, 0.3);
        }

        color
    }

    fn get_voxel_value(&self, generator: &NoiseGenerator, pos: Vec3) -> f32 {
        let noise_v = if pos.y <= 1.5 {
            // println!("{pos}");
            f32::MAX
        }else{
            f32::MIN
        };
        noise_v
    }

    fn try_generate_object(&self, _pos: Vec3, _value: f32) -> Option<Object> {
        None
    }
}

impl PlainBiomeGenerator {
    pub fn new() -> Self {
        Self {
            layer2d: Plain2dLayer::default(),
        }
    }
}
