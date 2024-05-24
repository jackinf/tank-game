use bevy::prelude::{AlphaMode, Asset, Color, ColorMaterial, Material, TypePath};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    alpha_mode: AlphaMode,
}

impl From<CustomMaterial> for ColorMaterial {
    fn from(custom_material: CustomMaterial) -> Self {
        ColorMaterial {
            color: custom_material.color,
            texture: None,
        }
    }
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/edge_highlight_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
